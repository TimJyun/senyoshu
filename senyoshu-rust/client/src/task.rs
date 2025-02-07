use std::sync::Mutex;

//js世界的回调不能访问 global_signal 所以才需要任务队列
pub static TASK_DEQUE: TaskDeque = TaskDeque::new();

pub struct TaskDeque(Mutex<Vec<Box<dyn FnOnce() + 'static + Send>>>);

impl TaskDeque {
    pub fn is_some(&self) -> Option<bool> {
        let vec = self.0.try_lock().ok()?;
        Some(vec.is_empty() == false)
    }

    const fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }

    pub fn add(&self, callback: impl FnOnce() + 'static + Send) {
        let mut tasks = self.0.lock().unwrap();
        tasks.push(Box::new(callback));
    }

    pub fn exec(&self) {
        let mut tasks = self.0.lock().unwrap();
        while let Some(task) = tasks.pop() {
            task();
        }
    }
}
