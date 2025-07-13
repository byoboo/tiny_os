//! Task Queue and List Management
//!
//! This module provides queue and list structures for managing tasks
//! in the scheduler with no_std compatibility.

use super::task::{Task, TaskId};

/// Simple array-based queue for tasks (no_std compatible)
#[derive(Debug, Clone)]
pub struct TaskQueue {
    tasks: [Option<Task>; 16], // Maximum 16 tasks per priority level
    head: usize,
    tail: usize,
    count: usize,
}

impl TaskQueue {
    /// Create a new empty task queue
    pub const fn new() -> Self {
        const NONE_TASK: Option<Task> = None;
        Self {
            tasks: [NONE_TASK; 16],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Add a task to the back of the queue
    pub fn push_back(&mut self, task: Task) -> Result<(), &'static str> {
        if self.count >= 16 {
            return Err("Task queue full");
        }

        self.tasks[self.tail] = Some(task);
        self.tail = (self.tail + 1) % 16;
        self.count += 1;
        Ok(())
    }

    /// Remove a task from the front of the queue
    pub fn pop_front(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        let task = self.tasks[self.head].take();
        self.head = (self.head + 1) % 16;
        self.count -= 1;
        task
    }

    /// Get the number of tasks in the queue
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get an iterator over the tasks in the queue
    pub fn iter(&self) -> TaskQueueIter<'_> {
        TaskQueueIter {
            queue: self,
            index: 0,
        }
    }

    /// Remove a specific task by ID
    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        let mut current = self.head;

        for _ in 0..self.count {
            if let Some(ref task) = self.tasks[current] {
                if task.id == task_id {
                    let removed_task = self.tasks[current].take();

                    // Shift elements to fill the gap
                    let mut shift_pos = current;
                    for _ in 0..(self.count - 1) {
                        let next_pos = (shift_pos + 1) % 16;
                        self.tasks[shift_pos] = self.tasks[next_pos].take();
                        shift_pos = next_pos;
                    }

                    self.count -= 1;
                    if self.count == 0 {
                        self.head = 0;
                        self.tail = 0;
                    } else {
                        self.tail = (self.tail + 15) % 16; // Move tail back
                    }

                    return removed_task;
                }
            }
            current = (current + 1) % 16;
        }

        None
    }
}

/// Iterator for TaskQueue
pub struct TaskQueueIter<'a> {
    queue: &'a TaskQueue,
    index: usize,
}

impl<'a> Iterator for TaskQueueIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.queue.count {
            return None;
        }

        let pos = (self.queue.head + self.index) % 16;
        self.index += 1;

        self.queue.tasks[pos].as_ref()
    }
}

/// Simple task list for no_std environment
#[derive(Debug)]
pub struct TaskList {
    tasks: [Option<Task>; 32],
    count: usize,
}

impl TaskList {
    /// Create a new empty task list
    pub const fn new() -> Self {
        const NONE_TASK: Option<Task> = None;
        Self {
            tasks: [NONE_TASK; 32],
            count: 0,
        }
    }

    /// Add a task to the list
    pub fn push(&mut self, task: Task) -> Result<(), &'static str> {
        if self.count >= 32 {
            return Err("Task list full");
        }

        for slot in &mut self.tasks {
            if slot.is_none() {
                *slot = Some(task);
                self.count += 1;
                return Ok(());
            }
        }

        Err("No free slots")
    }

    /// Remove and return any task from the list
    pub fn pop(&mut self) -> Option<Task> {
        for slot in &mut self.tasks {
            if let Some(task) = slot.take() {
                self.count -= 1;
                return Some(task);
            }
        }
        None
    }

    /// Remove a specific task by ID
    pub fn remove(&mut self, task_id: TaskId) -> Option<Task> {
        for slot in &mut self.tasks {
            if let Some(task) = slot {
                if task.id == task_id {
                    let removed_task = slot.take().unwrap();
                    self.count -= 1;
                    return Some(removed_task);
                }
            }
        }
        None
    }

    /// Get the number of tasks in the list
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get an iterator over the tasks in the list
    pub fn iter(&self) -> TaskListIter<'_> {
        TaskListIter {
            list: self,
            index: 0,
        }
    }

    /// Find a task by ID
    pub fn find(&self, task_id: TaskId) -> Option<&Task> {
        for slot in &self.tasks {
            if let Some(task) = slot {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }
        None
    }

    /// Find a mutable task by ID
    pub fn find_mut(&mut self, task_id: TaskId) -> Option<&mut Task> {
        for slot in &mut self.tasks {
            if let Some(task) = slot {
                if task.id == task_id {
                    return Some(task);
                }
            }
        }
        None
    }
}

/// Iterator for TaskList
pub struct TaskListIter<'a> {
    list: &'a TaskList,
    index: usize,
}

impl<'a> Iterator for TaskListIter<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 32 {
            if let Some(ref task) = self.list.tasks[self.index] {
                self.index += 1;
                return Some(task);
            }
            self.index += 1;
        }
        None
    }
}
