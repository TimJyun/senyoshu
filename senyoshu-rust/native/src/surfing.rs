use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
use std::os::fd::FromRawFd;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use shadowsocks_service::acl::AccessControl;
use shadowsocks_service::config::{
    Config, ConfigType, LocalConfig, LocalInstanceConfig, ProtocolType, ServerInstanceConfig,
};
use shadowsocks_service::local;
use shadowsocks_service::shadowsocks::config::Mode;
use shadowsocks_service::shadowsocks::crypto::CipherKind;
use shadowsocks_service::shadowsocks::{ServerAddr, ServerConfig};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tracing::{error, info};
use tun2proxy::{ArgDns, ArgProxy, ArgVerbosity, Args, CancellationToken, ProxyType};

use senyoshu_common::types::api::api::{SurfServer, GET_SURF_SERVERS_API};

use crate::init::{PATH, REDB, TABLE};
use crate::tun_device::TunDevice;
use crate::HANDLES;

#[derive(Clone, uniffi::Record)]
pub struct ConfigLite {
    pub acl_path: String,
    pub tun_fd: i32,
}

pub(crate) const PROXY_ADDR: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 15, 73, 1)), 7345);

#[uniffi::export]
pub fn surfing(config_lite: ConfigLite) {
    stop_surfing();
    if let Some(server) = get_current_server() {
        let socks_handle = spawn_socks(server, &config_lite.acl_path);
        let (tun_handle, cancel_token) = tun2socks(config_lite.tun_fd);
        let mut handles = HANDLES.write().unwrap();
        handles.ss = Some(socks_handle);
        handles.tun2proxy = Some(tun_handle);
        handles.tun2proxy_token = Some(cancel_token);
    }
}

pub(crate) fn spawn_socks(server: SurfServerExport, acl_path: impl AsRef<str>) -> JoinHandle<()> {
    let mut config = Config::new(ConfigType::Local);
    let acl = AccessControl::load_from_file(acl_path.as_ref());
    let acl = acl
        .map_err(|err| {
            error!("load acl from file err : {}", err);
        })
        .ok();
    config.server.push(ServerInstanceConfig {
        config: ServerConfig::new(
            ServerAddr::SocketAddr(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::from_str(server.server.as_str()).unwrap()),
                server.server_port,
            )),
            server.password,
            CipherKind::from_str(server.method.as_str()).unwrap(),
        ),
        acl: acl.clone(),
        outbound_fwmark: None,
        outbound_bind_addr: None,
        outbound_bind_interface: None,
    });

    config.local.push(LocalInstanceConfig {
        config: LocalConfig {
            addr: Some(ServerAddr::SocketAddr(PROXY_ADDR)),
            protocol: ProtocolType::Socks,
            mode: Mode::TcpAndUdp,
            udp_addr: None,
            ipv6_only: false,
            socks5_auth: Default::default(),
        },
        acl: acl.clone(),
    });
    config.udp_mtu = Some(1500);
    HANDLES.spawn(async move {
        info!("try run ss service . ");
        let run_rv = local::run(config).await;
        if let Err(err) = run_rv {
            error!("{}", err);
        } else {
            info!("ss service close . ");
        }
    })
}

pub(crate) fn tun2socks(tun_fd: i32) -> (JoinHandle<()>, CancellationToken) {
    info!("try run tun2proxy . ");

    let cancel_token = CancellationToken::new();
    let cancel_token_rv = cancel_token.clone();
    (
        HANDLES.spawn(async move {
            let run = tun2proxy::run(
                TunDevice::from_raw_fd(tun_fd),
                1500,
                Args {
                    proxy: ArgProxy {
                        proxy_type: ProxyType::Socks5,
                        addr: PROXY_ADDR,
                        credentials: None,
                    },
                    tun: None,
                    tun_fd: None,
                    ipv6_enabled: false,
                    setup: false,
                    dns: ArgDns::OverTcp,
                    dns_addr: IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                    bypass: vec![
                        "127.9.8.7".parse().unwrap(),
                        "127.15.73.1".parse().unwrap(),
                        "119.29.29.29".parse().unwrap(),
                        "10.0.0.1".parse().unwrap(),
                    ],
                    tcp_timeout: 600,
                    verbosity: ArgVerbosity::Trace,
                },
                cancel_token,
            );
            if let Err(err) = run.await {
                error!("run tun2proxy error : {}", err);
            } else {
                info!("tun2proxy exit . ");
            }
        }),
        cancel_token_rv,
    )
}

#[uniffi::export]
pub fn is_surfing() -> bool {
    let handles = HANDLES.read().unwrap();
    handles.ss.is_some() && handles.tun2proxy.is_some() && handles.tun2proxy_token.is_some()
}

#[uniffi::export]
pub fn stop_surfing() {
    let mut handles = HANDLES.write().unwrap();
    if let Some(ref ss_handle) = handles.ss {
        ss_handle.abort();
    }
    handles.ss = None;

    if let Some(ref cancel_token) = handles.tun2proxy_token {
        cancel_token.cancel();
    }
    handles.tun2proxy_token = None;

    if let Some(ref tun2proxy_handle) = handles.tun2proxy {
        tun2proxy_handle.abort();
    }
    handles.tun2proxy = None;
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, uniffi::Record)]
pub struct SurfServerExport {
    pub name: String,
    pub server: String,
    pub server_port: u16,
    pub password: String,
    pub method: String,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub delay: Option<i64>,
}

impl PartialEq for SurfServerExport {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.server == other.server
            && self.server_port == other.server_port
            && self.password == other.password
            && self.method == other.method
    }
}

pub(crate) static SERVERS: RwLock<Vec<SurfServerExport>> = RwLock::new(Vec::new());

static LOADED: AtomicBool = AtomicBool::new(false);

#[uniffi::export]
pub fn init_servers_list() {
    if LOADED.swap(true, Ordering::SeqCst) == false {
        HANDLES.spawn(async {
            (|| {
                let servers_from_db = {
                    let read_txn = REDB.begin_read().ok()?;
                    let mut table = read_txn.open_table(TABLE).ok()?;
                    let guard = table.get("servers").ok()??;
                    serde_json::from_str::<Vec<SurfServerExport>>(guard.value().as_str()).ok()?
                };
                let mut servers = SERVERS.write().unwrap();
                *servers = servers_from_db;
                Some(())
            })();
            if let Ok(servers_from_api) = GET_SURF_SERVERS_API.call(&()).await {
                let servers_new = servers_from_api
                    .into_iter()
                    .map(|ss| SurfServerExport {
                        name: ss.name,
                        server: ss.server,
                        server_port: ss.server_port,
                        password: ss.password,
                        method: ss.method,
                        delay: None,
                    })
                    .collect::<Vec<_>>();
                (|| {
                    let write_txn = REDB.begin_write().ok()?;
                    {
                        let mut table = write_txn.open_table(TABLE).ok()?;
                        let mut servers = SERVERS.write().unwrap();
                        let servers_new_json = serde_json::to_string(&servers_new);
                        *servers = servers_new;
                        table.insert("servers", servers_new_json.ok()?).ok()?;
                    }
                    write_txn.commit().ok()?;
                    Some(())
                })();
            }
            check_delay();
        });
    }
}

#[uniffi::export]
pub fn check_delay() {
    HANDLES.spawn(async {
        let mut servers_to_be_checked = { SERVERS.read().unwrap().to_vec() };
        for server_to_be_check in servers_to_be_checked.into_iter() {
            tokio::spawn(async move {
                let time_before = Local::now();
                let delay = if let Ok(_tcp_stream) = TcpStream::connect(format!(
                    "{}:{}",
                    server_to_be_check.server, server_to_be_check.server_port
                ))
                .await
                {
                    let time_after = Local::now();
                    time_after.timestamp_millis() - time_before.timestamp_millis()
                } else {
                    30000
                };
                let mut servers_lock = SERVERS.write().unwrap();
                for server_lock in servers_lock.iter_mut() {
                    if server_lock.server == server_to_be_check.server
                        && server_lock.server_port == server_to_be_check.server_port
                    {
                        server_lock.delay = Some(delay);
                    }
                }
                if let Some(handler) = SERVER_CHANGE_HANDLER.read().unwrap().as_ref() {
                    let servers = servers_lock.to_vec();
                    handler.on_update(servers);
                }
            });
        }
    });
}

#[uniffi::export]
pub fn set_current_server(server: SurfServerExport) -> bool {
    let result = (|| {
        let write_txn = REDB.begin_write().ok()?;
        {
            let mut table = write_txn.open_table(TABLE).ok()?;
            table
                .insert("server", serde_json::to_string(&server).ok()?)
                .ok()?;
        }
        write_txn.commit().ok()?;
        Some(())
    })();
    result.is_some()
}

#[uniffi::export]
pub fn get_current_server() -> Option<SurfServerExport> {
    let read_txn = REDB.begin_read().ok()?;
    let mut table = read_txn.open_table(TABLE).ok()?;
    serde_json::from_str(table.get("server").ok()??.value().as_str()).ok()
}

static SERVER_CHANGE_HANDLER: RwLock<Option<Arc<dyn ServerChangeHandler>>> = RwLock::new(None);

#[uniffi::export]
pub fn set_server_change_handler(handler: Option<Arc<dyn ServerChangeHandler>>) {
    *SERVER_CHANGE_HANDLER.write().unwrap() = handler;
}

#[uniffi::export(with_foreign)]
pub trait ServerChangeHandler: Send + Sync + Debug {
    fn on_update(&self, servers: Vec<SurfServerExport>);
}
