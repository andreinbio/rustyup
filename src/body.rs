use orbtk::prelude::*;

pub struct Body {
    tabs: Vec<Entity>,
    entity: Entity,
}

impl Body {
    pub fn init(ctx: &mut BuildContext) -> Self {
        let mut tabs: Vec<Entity> = vec![];
        let mut grid: Grid = Grid::create()
            .attach(Grid::row(1));

        let colors: [&str;3] = ["settings", "tab1", "tab2"];

        for index in 0..3 {
            let visibility = if index == 0 {
                "visible"
            } else {
                "collapsed"
            };

            let child_grid = Grid::create()
                .selector(colors[index])
                .visibility(visibility)
                .build(ctx);

            tabs.push(child_grid.clone());

            grid = grid.child(
                child_grid
            );
        }

        Body {
            tabs: tabs,
            entity: grid.build(ctx)
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.entity.clone()
    }

    pub fn get_tabs(&self) -> Vec<Entity> {
        self.tabs.clone()
    }
}
