use bevy::prelude::*;

const NORMAL_COLOR: UiColor = UiColor(Color::rgb(0.0, 0.28, 0.98));
const HOVERED_COLOR: UiColor = UiColor(Color::rgb(0.01, 0.46, 0.99));
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    
    commands.spawn_bundle(ButtonBundle{
        style: Style{
            size: Size::new(Val::Px(200.0), Val::Px(70.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: NORMAL_COLOR.into(),
        ..Default::default()
    }).with_children(|parent|{
        parent.spawn_bundle(TextBundle{
            text: Text::with_section("Start",
            TextStyle { font: asset_server.load("fonts/FreeMonoBold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9) 
            },
            Default::default()),
            visibility: Visibility{
                is_visible: true,
            },
            ..Default::default()
        });
    });
}

pub fn button_system(mut interaction_query: Query<
    (&Interaction, &mut UiColor, &mut Visibility, &Children),
    (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>
)
{
    for (interaction, mut color,
        mut is_visible, children) in interaction_query.iter_mut(){
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction{
            Interaction::Clicked => {
                *is_visible = Visibility{
                    is_visible: false,
                };
                text.sections[0].value = String::from("");
            }
            Interaction::Hovered => {
                *color = HOVERED_COLOR;
            }
            Interaction::None => {
                *color = NORMAL_COLOR;
            }
        };
    }
}
