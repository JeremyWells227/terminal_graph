
use crate::ui::{
        Menu,
        //GraphBox
};
pub struct App<'a> { 
    pub title:  String,
    pub menu:   Menu<'a>
}
impl<'a>  App<'a> {
    pub fn new(
        title: String,
        menu: Menu<'a>,

        ) -> App<'a> {




        App{
            title: title,
            menu
        }
    }

}


