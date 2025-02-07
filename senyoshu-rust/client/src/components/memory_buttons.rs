use dioxus::core_macro::Props;
use dioxus::prelude::*;
use smallvec::SmallVec;
use tracing::log::error;

use senyoshu_common::types::learn::learn_knowledge_history::{OperateRecord, OperateType};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::util::time::UtcTimeStamp;

#[derive(Props, PartialEq, Clone)]
pub struct MemoryButtonsProps {
    on_select: EventHandler<OperateRecord>,
    plan: Option<Plan>,

    forget_button: Option<bool>,
    seen_button: Option<bool>,
}

pub fn MemoryButtons(props: MemoryButtonsProps) -> Element {
    let mut buttons: SmallVec<[Element; 4]> = SmallVec::new();

    let on_select = props.on_select;

    fn display_next_review_time(next_review_time: UtcTimeStamp) -> String {
        const TIME_UNIT: [&str; 5] = ["min", "hour", "day", "week", "month"];
        let mut time_unit = 0;
        let mut time = (next_review_time.0 - UtcTimeStamp::now().0) / 1000 / 60;
        while time >= 100 && time_unit < 4 {
            match time_unit {
                0 => time = time / 60,
                1 => {
                    time = time / 24;
                }
                2 => {
                    time = time / 7;
                }
                3 => {
                    time = time * 7 / 30;
                }
                _ => {
                    error!("time_unit error:{}", time_unit)
                }
            }
            time_unit = time_unit + 1
        }
        format!("{time} {}s later", TIME_UNIT[time_unit])
    }

    let remember_next_review_time_node = props.plan.to_owned().map(|mut plan| {
        plan.calculate(Vec::from([OperateRecord {
            operate_type: OperateType::Remember,
            operate_time: UtcTimeStamp::now(),
        }]));
        rsx! {
            " ("
            {display_next_review_time(plan.next_review_time)},
            ")"
        }
    });

    buttons.push(rsx! {
        span {
            style: "flex:1;height:32px;line-height:32px;margin:8px;border: 1px;border-style: solid;border-radius: 12px",
            onclick: {
                let on_select = on_select.to_owned();
                move |_| {
                    on_select
                        .call(OperateRecord {
                            operate_type: OperateType::Remember,
                            operate_time: UtcTimeStamp::now(),
                        });
                }
            },
            "remember"
            {remember_next_review_time_node}
        }
    });

    let vague_next_review_time_node = props.plan.to_owned().map(|mut plan| {
        plan.calculate(Vec::from([OperateRecord {
            operate_type: OperateType::Vague,
            operate_time: UtcTimeStamp::now(),
        }]));

        rsx! {
            " ("
            {display_next_review_time(plan.next_review_time)},
            ")"
        }
    });

    buttons.push(rsx! {
        span {
            style: "flex:1;height:32px;line-height:32px;margin:8px;border: 1px;border-style: solid;border-radius: 12px",
            onclick: {
                let on_select = on_select.to_owned();
                move |_| {
                    on_select
                        .call(OperateRecord {
                            operate_type: OperateType::Vague,
                            operate_time: UtcTimeStamp::now(),
                        });
                }
            },
            "vague"
            {vague_next_review_time_node}
        }
    });

    if props.forget_button.unwrap_or_default() {
        let forget_next_review_time_node = props.plan.to_owned().map(|mut plan| {
            plan.calculate(Vec::from([OperateRecord {
                operate_type: OperateType::Forget,
                operate_time: UtcTimeStamp::now(),
            }]));
            rsx! {
                " ("
                {display_next_review_time(plan.next_review_time)},
                ")"
            }
        });

        buttons.push(rsx! {
            span {
                style: "flex:1;height:32px;line-height:32px;margin:8px;border: 1px;border-style: solid;border-radius: 12px",
                onclick: {
                    let on_select = on_select.to_owned();
                    move |_| {
                        on_select
                            .call(OperateRecord {
                                operate_type: OperateType::Forget,
                                operate_time: UtcTimeStamp::now(),
                            });
                    }
                },
                "forget"
                {forget_next_review_time_node}
            }
        });
    }

    if props.seen_button.unwrap_or_default() {
        let seen_next_review_time_node = props.plan.to_owned().map(|mut plan| {
            plan.calculate(Vec::from([OperateRecord {
                operate_type: OperateType::Seen,
                operate_time: UtcTimeStamp::now(),
            }]));
            rsx! {
                " ("
                {display_next_review_time(plan.next_review_time)},
                ")"
            }
        });

        buttons.push(rsx! {
            span {
                style: "flex:1;height:32px;line-height:32px;margin:8px;border: 1px;border-style: solid;border-radius: 12px",
                onclick: move |_| {
                    on_select
                        .call(OperateRecord {
                            operate_type: OperateType::Seen,
                            operate_time: UtcTimeStamp::now(),
                        });
                },
                "seen"
                {seen_next_review_time_node}
            }
        });
    }

    rsx! {
        div { style: "
                    background-color: #fff;
                    display:flex;
                    flex-direction: row;
                    text-align:center;
                    width:100%;
                    user-select:none;
                    height:48px;
                    ",
            {buttons.into_iter()}
        }
    }
}
