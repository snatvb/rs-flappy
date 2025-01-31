use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard, OnceLock, RwLock, RwLockReadGuard};
use std::{cell::RefCell, rc::Rc};

use raylib::{RaylibHandle, RaylibThread};

mod renderer;
mod scene;

pub struct Engine {
    rl: Mutex<RaylibHandle>,
    thread: RwLock<RaylibThread>,
}

pub struct RaylibHandleGuard<'a> {
    handle: MutexGuard<'a, RaylibHandle>,
}

pub struct RaylibThreadGuard<'a> {
    guard: RwLockReadGuard<'a, RaylibThread>,
}

static INSTANCE: OnceLock<Engine> = OnceLock::new();

impl Engine {
    pub fn singleton() -> &'static Engine {
        INSTANCE.get().expect("Engine not initialized")
    }

    pub fn init(rl: RaylibHandle, thread: RaylibThread) -> &'static Engine {
        INSTANCE
            .set(Engine {
                rl: Mutex::new(rl),
                thread: RwLock::new(thread),
            })
            .unwrap_or_else(|_| panic!("Engine already initialized"));
        Self::singleton()
    }

    pub fn rl() -> RaylibHandleGuard<'static> {
        Self::singleton().rl_handle()
    }

    fn rl_handle(&self) -> RaylibHandleGuard<'_> {
        RaylibHandleGuard {
            handle: self.rl.lock().unwrap(),
        }
    }

    fn thread(&self) -> RaylibThreadGuard<'_> {
        RaylibThreadGuard {
            guard: self.thread.read().unwrap(),
        }
    }

    pub fn rl_thread() -> RaylibThreadGuard<'static> {
        Self::singleton().thread()
    }
}

impl<'a> std::ops::Deref for RaylibHandleGuard<'a> {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<'a> std::ops::DerefMut for RaylibHandleGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
    }
}

impl<'a> std::ops::Deref for RaylibThreadGuard<'a> {
    type Target = RaylibThread;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

// pub struct Engine {
//     pub rl: RaylibHandle,
//     pub thread: RaylibThread,
// }
//
// static mut INSTANCE: OnceLock<Engine> = OnceLock::new();
//
// impl Engine {
//     pub fn singleton() -> &'static mut Engine {
//         unsafe { INSTANCE.get_mut().unwrap_unchecked() }
//         // .expect("Engine not initialized. Call Engine::new() first.")
//     }
//
//     pub fn init(rl: RaylibHandle, thread: RaylibThread) -> &'static mut Engine {
//         unsafe {
//             INSTANCE.set(Engine { rl, thread }).unwrap_or_else(|_| {
//                 panic!("Engine already initialized");
//             });
//         }
//         Self::singleton()
//     }
//
//     pub fn rlh() -> &'static mut RaylibHandle {
//         &mut Self::singleton().rl
//     }
//
//     pub fn rl(&self) -> &'static mut RaylibHandle {
//         Engine::rlh()
//     }
//
//     pub fn thread(&'static self) -> &'static RaylibThread {
//         &self.thread
//     }
// }

unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}
