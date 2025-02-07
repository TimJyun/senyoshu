use serde::{Deserialize, Serialize};

use crate::types::learn::learn_knowledge_history::{OperateRecord, OperateType};
use crate::util::time::UtcTimeStamp;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Plan {
    pub exp: f64,
    pub last_review_time: UtcTimeStamp,
    pub next_review_time: UtcTimeStamp,
    pub last_seen_time: UtcTimeStamp,
}

impl Plan {
    pub fn calculate(&mut self, operate_vec: Vec<OperateRecord>) {
        for learn_operate in operate_vec.into_iter() {
            match learn_operate.operate_type {
                OperateType::Remember => {
                    if *self.last_review_time <= *learn_operate.operate_time {
                        let intervals =
                            (*learn_operate.operate_time - *self.last_review_time) as f64;
                        let review_interval_day = get_next_review_day_by_exp(self.exp);
                        let mut get_exp = (intervals / (*review_interval_day as f64)).min(3.);
                        if get_exp > 1. {
                            get_exp = 1. + (get_exp - 1.) / 2.;
                        }
                        self.exp = check_exp(self.exp + get_exp);
                        self.last_review_time = learn_operate.operate_time;
                        self.next_review_time =
                            learn_operate.operate_time + get_next_review_day_by_exp(self.exp);
                    } else {
                        panic!("operate_vec is not sorted ");
                    }
                }
                OperateType::Vague => {
                    if *self.last_review_time <= *learn_operate.operate_time {
                        if self.exp > 2. {
                            self.exp = 2.
                        } else {
                            self.exp = check_exp(self.exp - 1.);
                        }
                        self.last_review_time = learn_operate.operate_time;
                        self.next_review_time =
                            learn_operate.operate_time + get_next_review_day_by_exp(self.exp);
                    } else {
                        panic!("operate_vec is not sorted")
                    }
                }
                OperateType::Forget => {
                    if *self.last_review_time <= *learn_operate.operate_time {
                        self.exp = 0.;
                        self.last_review_time = learn_operate.operate_time;
                        self.next_review_time =
                            learn_operate.operate_time + get_next_review_day_by_exp(self.exp);
                    } else {
                        panic!("operate_vec is not sorted")
                    }
                }
                OperateType::Seen => {
                    if *self.last_seen_time <= *learn_operate.operate_time {
                        *self.next_review_time += (*learn_operate.operate_time
                            - *self.last_seen_time)
                            .max(0)
                            .min(*get_next_review_day_by_exp(self.exp))
                            / 2;
                    } else {
                        panic!("operate_vec is not sorted")
                    }
                }
            }

            if *self.last_seen_time < *learn_operate.operate_time {
                self.last_seen_time = learn_operate.operate_time;
            }
        }
    }
}

fn check_exp(exp: f64) -> f64 {
    exp.max(0.).min(7.)
}

fn get_next_review_day_by_exp(exp: f64) -> UtcTimeStamp {
    let level = check_exp(exp).floor() as u32;

    UtcTimeStamp(24i64 * 60 * 60 * 1000 * (2i64.pow(level + 1) - 1))
}
