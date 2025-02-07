use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local, Utc};
use gloo::storage::{LocalStorage, Storage};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::api::learn::{GET_RECORD_API, POST_LEARN_RECORD_API};
use senyoshu_common::types::learn::knowledge::Knowledge;
use senyoshu_common::types::learn::learn_knowledge_history::{
    LearnKnowledgeHistory, OperateRecord,
};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::types::learn::LearnHistoryMap;

use crate::storage::LAST_UPDATED;

//store modelLearnHistoryMap
#[derive(Default, Serialize, Deserialize)]
pub struct WorkBook {
    pub history: LearnHistoryMap,
    #[serde(default, skip_serializing_if = "HashSet::is_empty")]
    pub to_be_push: HashSet<Knowledge>,
}

const WORK_BOOK_LAST_SYNC_LOCAL_STORAGE: &str = "WorkBookLastSync";
const WORK_BOOK_LOCAL_STORAGE: &str = "WorkBook";

impl WorkBook {
    pub fn get_last_sync_time() -> Option<DateTime<Utc>> {
        LocalStorage::get::<Option<DateTime<Utc>>>(WORK_BOOK_LAST_SYNC_LOCAL_STORAGE)
            .ok()
            .flatten()
    }

    pub fn get() -> Self {
        LocalStorage::get::<WorkBook>(WORK_BOOK_LOCAL_STORAGE).unwrap_or_default()
    }

    pub fn with_mut(closure: impl FnOnce(&mut WorkBook)) {
        let mut tmp = Self::get();
        closure(&mut tmp);
        LocalStorage::set::<WorkBook>(WORK_BOOK_LOCAL_STORAGE, tmp)
            .expect("store work book failed");
    }

    pub fn plan() -> Vec<(Knowledge, Plan)> {
        let this = Self::get();
        let mut plans = this
            .history
            .iter()
            .filter(|(_, v)| v.freeze_time.is_none())
            .map(|(k, v)| {
                let mut plan = Plan::default();
                plan.calculate(v.history.to_owned());
                (k.to_owned(), plan)
            })
            .collect_vec();
        plans.sort_unstable_by_key(|(_know, plan)| Reverse(plan.next_review_time.0));
        plans
    }

    pub fn add_record(
        knowledge: Knowledge,
        operate_records: impl IntoIterator<Item = OperateRecord>,
    ) {
        Self::with_mut(|work_book| work_book.append_record(knowledge, operate_records));
    }

    pub fn append_record(
        &mut self,
        knowledge: Knowledge,
        operate_records: impl IntoIterator<Item = OperateRecord>,
    ) {
        self.to_be_push.insert(knowledge.to_owned());
        let tmp = self.history.entry(knowledge).or_default();

        let mut operate_records = operate_records.into_iter().collect::<Vec<_>>();
        operate_records.reverse();
        operate_records.append(&mut tmp.history);

        tmp.history = operate_records;
        tmp.history.sort();
        tmp.history.dedup();

        tmp.freeze_time = None;
    }

    pub async fn sync(token: Token) -> bool {
        let rv = Self::push(token.clone()).await && Self::update(token).await;
        if rv {
            let _ = LocalStorage::set::<Option<DateTime<Utc>>>(
                WORK_BOOK_LAST_SYNC_LOCAL_STORAGE,
                Some(Utc::now()),
            );
        }
        rv
    }

    async fn push(token: Token) -> bool {
        debug!("try post_learn_record");
        let mut success = 0;
        let work_book = Self::get();
        let count = work_book.to_be_push.len();
        if count > 0 {
            const GROUP_LEN: usize = 500;
            let mut groups: Vec<HashMap<Knowledge, LearnKnowledgeHistory>> =
                Vec::with_capacity((work_book.to_be_push.len() / GROUP_LEN) + 1);
            groups.push(Default::default());
            for k in work_book.to_be_push.to_owned().into_iter() {
                let h = work_book.history.get(&k).cloned().unwrap_or_default();
                if let Some(set) = groups.last_mut() {
                    if set.len() >= GROUP_LEN {
                        let mut to_be_push_map = HashMap::with_capacity(GROUP_LEN);
                        to_be_push_map.insert(k, h);
                        groups.push(to_be_push_map);
                    } else {
                        set.insert(k, h);
                    }
                } else {
                    error!("workbook::push err code : 0");
                }
            }

            for group in groups {
                if group.len() == 0 {
                    continue;
                }

                if POST_LEARN_RECORD_API
                    .call(&(token.to_owned(), LearnHistoryMap::new(group.to_owned())))
                    .await
                    .unwrap_or(false)
                {
                    //todo:这里不是原子操作
                    WorkBook::with_mut(|work_book: &mut WorkBook| {
                        for k in group.keys() {
                            work_book.to_be_push.remove(&k);
                        }
                    });

                    success += group.len();
                } else {
                    error!("post_learn_record: failed");
                }
            }

            let result = success == count;
            if result {
                debug!("post_learn_record: {}", success);
            } else {
                error!("post_learn_record: {}/{}", success, count);
            }
            result
        } else {
            true
        }
    }

    async fn update(token: Token) -> bool {
        let result = async {
            let last_update = { LAST_UPDATED.peek().workbook.to_owned() };
            let remote_history = GET_RECORD_API.call(&(token, last_update)).await.ok()??;
            let remote_history_count = remote_history.len();
            if remote_history_count > 0 {
                WorkBook::with_mut(|work_book| {
                    for (remote_know, remote_know_history) in remote_history.into_iter() {
                        work_book.append_record(
                            remote_know.to_owned(),
                            remote_know_history.history.to_owned(),
                        );
                        let new_freeze_time = remote_know_history.freeze_time.to_owned();
                        let mut local_know_history = work_book
                            .history
                            .entry(remote_know)
                            .or_insert(remote_know_history);
                        if let Some(new_freeze_time) = new_freeze_time {
                            if let Some(old_freeze_time) = local_know_history.freeze_time {
                                if new_freeze_time.timestamp_micros()
                                    > old_freeze_time.timestamp_micros()
                                {
                                    local_know_history.freeze_time = Some(new_freeze_time);
                                }
                            } else {
                                local_know_history.freeze_time = Some(new_freeze_time);
                            }
                        }
                    }
                });
            }
            let now = Local::now();
            LAST_UPDATED.write().workbook = Some(now.into());
            debug!("update_learn_record:{remote_history_count}");
            Some(())
        }
        .await
        .is_some();

        result
    }
}
