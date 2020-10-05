use bevy::prelude::*;

#[derive(Debug)]
pub struct TextFrame(pub Handle<Font>, pub bool);

#[derive(Debug)]
pub struct Fading {
    pub fade: f32,
    pub alpha: f32,
}

pub fn text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut texts: Query<(Entity, Mut<Fading>, Mut<Text>, &Parent, &CalculatedSize)>,
    mut frames: Query<With<TextFrame, (Entity, &Children, Mut<Draw>)>>,
    styles: Query<Mut<Style>>,
) {
    let delta_time = time.delta.as_secs_f32();

    for (e, children, mut draw) in &mut frames.iter() {
        draw.is_visible = !children.is_empty();
        styles.get_mut::<Style>(e).unwrap().size.height = Val::Px(0.0);
    }

    for (e, mut fading, mut text, parent, &size) in &mut texts.iter() {
        let width = Val::Px(size.size.width);
        styles.get_mut::<Style>(**parent).unwrap().size.width = width;
        match styles.get::<Style>(e).unwrap().size.height {
            Val::Px(height) => match &mut styles.get_mut::<Style>(**parent).unwrap().size.height {
                Val::Px(h) => *h += height,
                _ => {}
            },
            _ => {}
        }
        fading.alpha -= fading.fade * delta_time;
        if fading.alpha < 0.0 {
            commands.despawn_recursive(e);
        } else {
            text.style.color.a = fading.alpha;
        }
    }
}
