use std::iter::Iterator;
use std::array;
use std::ops::Deref;
use tui:: {
    backend::Backend,
    layout::{Rect,Alignment,Layout,Direction,Constraint},
    style::{Style,Modifier,Color},
    text::{Span,Text},
    widgets::{Block,Borders,List,ListItem,Widget,ListState,Paragraph},
    Frame
};



#[derive(Debug)]
pub struct Menu<'a>
{ 
    items: &'a Vec<Vec<String>>,
    selected:   SelectedItem,
    gui_items: Vec<Vec<ListItem<'a>>>,
    max_row: usize,
    max_col: Vec<usize>,

} 

impl<'a> Menu<'a>
{ 
    pub fn new(
        items: &'a Vec<Vec<String>>,
        ) -> Menu<'a> { 
        let mut selected = SelectedItem::new(0,0);

        let gui_items = Menu::build_gui(items,selected);


        let mut max_row = 0;
        

        println!("Item Length! {}",items.len());
        max_row = items.len()-1;
        let mut max_col : Vec<usize> = Vec::new();
        let mut i=0;
        for row in items { 
                max_col[i] = row.len()
        }


        return Menu
        {
            items,
            selected,
            gui_items,
            max_row,
            max_col,
        };
    }

    fn build_gui(items: &'a Vec<Vec<String>>,selected: SelectedItem) -> Vec<Vec<ListItem<'a>>>{
        let mut gui_items = Vec::new();
        let mut i=0;
        let mut j=0;
        for row in items {
            let mut list_row = Vec::new();
            gui_items.push(list_row);
            for item in row { 
                let normal_style: Style = Style::default();
                let selected_style: Style = Style::default().fg(Color::Black).bg(Color::Gray).add_modifier(Modifier::BOLD);
                let text_item = Text::raw(item);
                let mut list_item;
                if i == selected.x && j == selected.y {
                    println!("Highlighting {}, {}", i, j);
                    list_item = ListItem::new(text_item).style(selected_style);
                } else {
                    list_item = ListItem::new(text_item).style(normal_style);
                }
                list_item = list_item;
                gui_items[i].push(list_item);
                j=j+1;
            }
            i=i+1;
        }
        return gui_items
    }

    fn select_item(& mut self,x: usize, y: usize) {
        //self.unhighlight_item();
        //
        println!("Selecting {}x {}",x,y);
        self.selected =
            SelectedItem{
                x,
                y,
            };
        self.gui_items = Menu::build_gui(self.items,self.selected);
        //self.highlight_item();
    }

    pub fn move_left(&mut self) {
        if self.selected.x > 0 {
            self.select_item(self.selected.x-1,self.selected.y)
        };
    }
    pub fn move_right(&mut self) {
        if self.selected.x < self.max_row  {
            self.select_item(self.selected.x+1,self.selected.y)
        };
    }

    pub fn move_down(&mut self) {
        if self.selected.y < self.max_col[self.selected.x] {
            self.select_item(self.selected.x,self.selected.y+1)
        };
    }
    pub fn move_up(&mut self)  {
        if self.selected.y > 0 {
            self.select_item(self.selected.x,self.selected.y-1)
        };
    }

    pub fn ui<B: Backend>(&mut self,f: &mut Frame<B>, r: Rect)   {
        self.gui_items = Menu::build_gui(self.items,self.selected);
        let num_cols = self.gui_items.len() as u16;
        let col_per = 100/num_cols;
        let mut constraints: Vec<Constraint> = Vec::new();
        let mut per = col_per;
        while per <= 100 {
            constraints.push(Constraint::Percentage(col_per));
            per+=col_per;
        };


        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(5)
            .constraints(
                constraints
                )
            .split(r);
        
        let mut chunk_index =0;

        let  colors = [
            Color::Blue,
            Color::LightRed,
        ];
        for row in self.gui_items.iter() {
            let list = List::new(row.deref()).block(Block::default()).style(Style::default().bg(colors[chunk_index]));
            if chunk_index > chunks.len() -1 {
                f.render_widget(Paragraph::new("Chunk index out of bounds"),chunks[0])
            } {
                f.render_widget(list,chunks[chunk_index]);
            }
            chunk_index=chunk_index+1;
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct SelectedItem { 
    x: usize,
    y: usize,
}

impl SelectedItem { 
    pub fn new(x: usize,y: usize) -> SelectedItem {
        SelectedItem{
            x,
            y
        }
    }

}

