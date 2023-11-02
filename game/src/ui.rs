use druid::widget;
use druid::widget::Label;

use crate::data::BlackJackState;

pub fn ui_builder() -> impl Widget<BlackJackStateState>{
    Label::new("Hello world");
}