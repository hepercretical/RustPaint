use mspaint_port::canvas::{Canvas, Tool};
#[test]
fn paletted_and_bitfield_bitmap_inputs_decode_their_actual_channels() {
    let mut bmp = vec![0u8; 66];
    bmp[..2].copy_from_slice(b"BM");
    bmp[10..14].copy_from_slice(&62u32.to_le_bytes());
    bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
    bmp[18..22].copy_from_slice(&2u32.to_le_bytes());
    bmp[22..26].copy_from_slice(&1u32.to_le_bytes());
    bmp[26] = 1;
    bmp[28] = 1;
    bmp[54..58].copy_from_slice(&[0, 0, 0, 0]);
    bmp[58..62].copy_from_slice(&[0, 0, 255, 0]);
    bmp[62] = 0x40;
    assert_eq!(Canvas::from_bmp(&bmp).unwrap().pixels(), [0, 0xff0000]);
    let mut bmp = vec![0u8; 70];
    bmp[..2].copy_from_slice(b"BM");
    bmp[10..14].copy_from_slice(&66u32.to_le_bytes());
    bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
    bmp[18..22].copy_from_slice(&1u32.to_le_bytes());
    bmp[22..26].copy_from_slice(&1u32.to_le_bytes());
    bmp[26] = 1;
    bmp[28] = 16;
    bmp[30] = 3;
    bmp[54..58].copy_from_slice(&0xf800u32.to_le_bytes());
    bmp[58..62].copy_from_slice(&0x7e0u32.to_le_bytes());
    bmp[62..66].copy_from_slice(&0x1fu32.to_le_bytes());
    bmp[66..68].copy_from_slice(&0x7e0u16.to_le_bytes());
    assert_eq!(Canvas::from_bmp(&bmp).unwrap().pixels(), [0x00ff00]);
}
#[test]
fn brush_has_round_edges_and_keeps_its_center_color() {
    let mut canvas = Canvas::new(12, 12).unwrap();
    canvas.draw(Tool::Brush, (6, 6), (6, 6), 0x123456, 8);
    assert_eq!(canvas.get(6, 6), Some(0x123456));
    assert_eq!(canvas.get(2, 2), Some(0xffffff));
}
#[test]
fn transforms_and_history_restore_dimensions_and_pixels() {
    let mut canvas = Canvas::from_pixels(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
    canvas.rotate(1);
    assert_eq!((canvas.width(), canvas.height()), (2, 3));
    assert_eq!(canvas.pixels(), [4, 1, 5, 2, 6, 3]);
    assert!(canvas.undo());
    assert_eq!(canvas.pixels(), [1, 2, 3, 4, 5, 6]);
    assert_eq!((canvas.width(), canvas.height()), (3, 2));
    assert!(canvas.redo());
    assert_eq!(canvas.pixels(), [4, 1, 5, 2, 6, 3]);
    canvas.flip(true);
    assert_eq!(canvas.pixels(), [1, 4, 2, 5, 3, 6]);
    canvas.rotate(3);
    assert_eq!(canvas.pixels(), [4, 5, 6, 1, 2, 3]);
}
#[test]
fn crop_resize_and_canvas_extension_are_undoable() {
    let mut canvas = Canvas::from_pixels(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
    canvas.crop((1, 0), (2, 1)).unwrap();
    assert_eq!(canvas.pixels(), [2, 3, 5, 6]);
    canvas.resize(4, 4, true).unwrap();
    assert_eq!(canvas.get(3, 3), Some(6));
    canvas.undo();
    assert_eq!((canvas.width(), canvas.height()), (2, 2));
    canvas.undo();
    assert_eq!(canvas.pixels(), [1, 2, 3, 4, 5, 6]);
    canvas.resize(4, 3, false).unwrap();
    assert_eq!(canvas.get(2, 1), Some(6));
    assert_eq!(canvas.get(3, 2), Some(0xffffff));
}
#[test]
fn paste_expands_and_move_preview_has_one_undo_step() {
    let mut canvas = Canvas::new(2, 2).unwrap();
    let image = Canvas::from_pixels(2, 1, vec![0x112233, 0x445566]).unwrap();
    canvas.paste_expanding(&image, 1, 1).unwrap();
    assert_eq!((canvas.width(), canvas.height()), (3, 2));
    assert_eq!(canvas.get(2, 1), Some(0x445566));
    canvas.undo();
    assert_eq!((canvas.width(), canvas.height()), (2, 2));
    canvas.paste(&image, 0, 0);
    canvas.checkpoint();
    let bounds = canvas.move_selection_preview((0, 0), (1, 0), 0, 1).unwrap();
    assert_eq!(bounds, ((0, 1), (1, 1)));
    assert_eq!(canvas.get(0, 0), Some(0xffffff));
    assert_eq!(canvas.get(0, 1), Some(0x112233));
    canvas.undo();
    assert_eq!(canvas.get(0, 0), Some(0x112233));
}
#[test]
fn invalid_dimensions_and_off_canvas_regions_do_not_corrupt_document() {
    let mut canvas = Canvas::new(2, 2).unwrap();
    let before = canvas.pixels().to_vec();
    assert!(canvas.resize(u32::MAX, 2, true).is_err());
    assert_eq!(canvas.pixels(), before);
    let region = canvas
        .region((i32::MAX, i32::MAX), (i32::MAX, i32::MAX))
        .unwrap();
    assert_eq!((region.width(), region.height()), (1, 1));
}
#[test]
fn document_roundtrips_with_source_dib_size_and_top_down_pixels() {
    let mut canvas = Canvas::new(7, 5).unwrap();
    canvas.checkpoint();
    canvas.draw(Tool::Line, (0, 0), (6, 4), 0x123456, 1);
    let bmp = canvas.bmp().unwrap();
    assert_eq!(bmp.len(), 54 + 7 * 5 * 4);
    assert_eq!(&bmp[54..58], &[0x56, 0x34, 0x12, 0]);
    let loaded = Canvas::from_bmp(&bmp).unwrap();
    assert_eq!(loaded.pixels(), canvas.pixels());
    assert!(canvas.undo());
    assert!(canvas.pixels().iter().all(|c| *c == 0xffffff));
    assert!(canvas.redo());
    assert_eq!(loaded.pixels(), canvas.pixels());
}
#[test]
fn enclosed_fill_stays_within_rectangle_and_eraser_is_white() {
    let mut canvas = Canvas::new(12, 12).unwrap();
    canvas.draw(Tool::Rectangle, (2, 2), (9, 9), 0, 1);
    canvas.draw(Tool::Fill, (4, 4), (4, 4), 0xff0000, 1);
    assert_eq!(canvas.get(4, 4), Some(0xff0000));
    assert_eq!(canvas.get(2, 4), Some(0));
    assert_eq!(canvas.get(0, 0), Some(0xffffff));
    canvas.draw(Tool::Eraser, (4, 4), (4, 4), 0, 1);
    assert_eq!(canvas.get(4, 4), Some(0xffffff));
}
#[test]
fn preview_replaces_prior_shape_and_new_edit_discards_redo() {
    let mut canvas = Canvas::new(10, 10).unwrap();
    canvas.checkpoint();
    canvas.draw(Tool::Line, (0, 0), (9, 0), 0, 1);
    canvas.restore_preview();
    canvas.draw(Tool::Line, (0, 0), (0, 9), 0, 1);
    assert_eq!(canvas.get(9, 0), Some(0xffffff));
    assert_eq!(canvas.get(0, 9), Some(0));
    assert!(canvas.undo());
    canvas.checkpoint();
    assert!(!canvas.redo());
}
#[test]
fn bmp_import_checks_stride_orientation_and_truncation() {
    let mut bytes = vec![0; 62];
    bytes[..2].copy_from_slice(b"BM");
    bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
    bytes[14..18].copy_from_slice(&40u32.to_le_bytes());
    bytes[18..22].copy_from_slice(&1u32.to_le_bytes());
    bytes[22..26].copy_from_slice(&2u32.to_le_bytes());
    bytes[26] = 1;
    bytes[28] = 24;
    bytes[54..58].copy_from_slice(&[0, 0, 255, 0]);
    bytes[58..62].copy_from_slice(&[255, 0, 0, 0]);
    let canvas = Canvas::from_bmp(&bytes).unwrap();
    assert_eq!(canvas.get(0, 0), Some(0x0000ff));
    assert_eq!(canvas.get(0, 1), Some(0xff0000));
    assert!(Canvas::from_bmp(&bytes[..61]).is_err());
    assert!(Canvas::new(0, 10).is_err());
    assert!(Canvas::new(u32::MAX, 2).is_err());
}
