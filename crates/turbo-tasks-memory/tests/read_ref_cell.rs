#![feature(min_specialization)]

use std::sync::Mutex;

use anyhow::Result;
use turbo_tasks::{get_invalidator, Invalidator, ReadRef};
use turbo_tasks_testing::{register, run};

register!();

#[tokio::test]
async fn read_ref() {
    run! {
        let counter = CounterVc::cell(Counter { value: Mutex::new((0, None))});

        let counter_value = counter.get_value();

        assert_eq!(*counter.get_value().strongly_consistent().await?, 0);
        assert_eq!(*counter_value.strongly_consistent().await?, 0);

        counter.await?.incr();

        assert_eq!(*counter.get_value().strongly_consistent().await?, 1);
        assert_eq!(*counter_value.strongly_consistent().await?, 1);

        // `ref_counter` will still point to the same `counter` instance as `counter`.
        let ref_counter = ReadRef::cell(counter.await?);
        let ref_counter_value = ref_counter.get_value();

        // However, `local_counter_value` will point to the value of `counter_value`
        // at the time it was turned into a trait reference (just like a `ReadRef` would).
        let local_counter_value = ReadRef::cell(counter_value.await?).get_value();

        counter.await?.incr();

        assert_eq!(*counter.get_value().strongly_consistent().await?, 2);
        assert_eq!(*counter_value.strongly_consistent().await?, 2);
        assert_eq!(*ref_counter_value.strongly_consistent().await?, 2);
        assert_eq!(*local_counter_value.strongly_consistent().await?, 1);
    }
}

#[turbo_tasks::value(transparent)]
struct CounterValue(usize);

#[turbo_tasks::value(serialization = "none", cell = "new", eq = "manual")]
struct Counter {
    #[turbo_tasks(debug_ignore, trace_ignore)]
    value: Mutex<(usize, Option<Invalidator>)>,
}

impl Counter {
    fn incr(&self) {
        let mut lock = self.value.lock().unwrap();
        lock.0 += 1;
        if let Some(i) = lock.1.take() {
            i.invalidate();
        }
    }
}

#[turbo_tasks::value_impl]
impl CounterVc {
    #[turbo_tasks::function]
    async fn get_value(self) -> Result<CounterValueVc> {
        let this = self.await?;
        let mut lock = this.value.lock().unwrap();
        lock.1 = Some(get_invalidator());
        Ok(CounterValueVc::cell(lock.0))
    }
}

#[turbo_tasks::value_impl]
impl CounterValueVc {
    #[turbo_tasks::function]
    fn get_value(self) -> CounterValueVc {
        self
    }
}
