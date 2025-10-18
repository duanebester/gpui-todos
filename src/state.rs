use gpui::*;

use crate::todo::TodoItem;

#[derive(Clone)]
pub struct State {
    pub count: usize, // hack for generating todo item ids
    pub items: Vec<TodoItem>,
}

#[derive(Clone)]
pub struct StateModel {
    pub inner: Entity<State>,
}

impl StateModel {
    pub fn init(app: &mut App) {
        let model = app.new(|_cx| State {
            count: 0,
            items: vec![],
        });
        let this = Self { inner: model };
        app.set_global(this.clone());
    }

    pub fn update(f: impl FnOnce(&mut Self, &mut App), cx: &mut App) {
        if !cx.has_global::<Self>() {
            return;
        }
        cx.update_global::<Self, _>(|this, cx| {
            f(this, cx);
        });
    }

    pub fn push(&self, item: TodoItem, cx: &mut App) {
        self.inner.update(cx, |model, cx| {
            model.items.push(item.clone());
            model.count += 1;
            cx.emit(ListChangedEvent {});
        });
    }

    pub fn remove(&self, id: usize, cx: &mut App) {
        self.inner.update(cx, |model, cx| {
            let index = model.items.iter().position(|x| x.id == id).unwrap();
            model.items.remove(index);
            cx.emit(ListChangedEvent {});
        });
    }
}

impl Global for StateModel {}

#[derive(Clone, Debug)]
pub struct ListChangedEvent {}

impl EventEmitter<ListChangedEvent> for State {}
