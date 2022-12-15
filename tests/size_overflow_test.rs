/**
 * Copyright (c) 2014-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
// @Generated by gentest/gentest.rb from gentest/fixtures/YGSizeOverflowTest.html
extern crate ordered_float;
extern crate yoga;

use yoga::*;

#[test]
fn test_nested_overflowing_child() {
    let mut config = Config::new();

    let mut root = Node::new_with_config(&mut config);
    root.set_width(StyleUnit::Point((100 as f32).into()));
    root.set_height(StyleUnit::Point((100 as f32).into()));

    let mut root_child0 = Node::new_with_config(&mut config);
    root_child0.set_min_width(StyleUnit::Auto);
    root_child0.set_min_height(StyleUnit::Auto);
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0.set_width(StyleUnit::Point((200 as f32).into()));
    root_child0_child0.set_min_width(StyleUnit::Auto);
    root_child0_child0.set_height(StyleUnit::Point((200 as f32).into()));
    root_child0_child0.set_min_height(StyleUnit::Auto);
    root_child0.insert_child(&mut root_child0_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0.get_layout_height());

    assert_eq!(0 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(200 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0.get_layout_height());

    assert_eq!(-100 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(200 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());
}

#[test]
fn test_nested_overflowing_child_in_constraint_parent() {
    let mut config = Config::new();

    let mut root = Node::new_with_config(&mut config);
    root.set_width(StyleUnit::Point((100 as f32).into()));
    root.set_height(StyleUnit::Point((100 as f32).into()));

    let mut root_child0 = Node::new_with_config(&mut config);
    root_child0.set_width(StyleUnit::Point((100 as f32).into()));
    root_child0.set_min_width(StyleUnit::Auto);
    root_child0.set_height(StyleUnit::Point((100 as f32).into()));
    root_child0.set_min_height(StyleUnit::Auto);
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0.set_width(StyleUnit::Point((200 as f32).into()));
    root_child0_child0.set_min_width(StyleUnit::Auto);
    root_child0_child0.set_height(StyleUnit::Point((200 as f32).into()));
    root_child0_child0.set_min_height(StyleUnit::Auto);
    root_child0.insert_child(&mut root_child0_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(100 as f32, root_child0.get_layout_height());

    assert_eq!(0 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(200 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(100 as f32, root_child0.get_layout_height());

    assert_eq!(-100 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(200 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());
}

#[test]
fn test_parent_wrap_child_size_overflowing_parent() {
    let mut config = Config::new();

    let mut root = Node::new_with_config(&mut config);
    root.set_width(StyleUnit::Point((100 as f32).into()));
    root.set_height(StyleUnit::Point((100 as f32).into()));

    let mut root_child0 = Node::new_with_config(&mut config);
    root_child0.set_width(StyleUnit::Point((100 as f32).into()));
    root_child0.set_min_width(StyleUnit::Auto);
    root_child0.set_min_height(StyleUnit::Auto);
    root.insert_child(&mut root_child0, 0);

    let mut root_child0_child0 = Node::new_with_config(&mut config);
    root_child0_child0.set_width(StyleUnit::Point((100 as f32).into()));
    root_child0_child0.set_min_width(StyleUnit::Auto);
    root_child0_child0.set_height(StyleUnit::Point((200 as f32).into()));
    root_child0_child0.set_min_height(StyleUnit::Auto);
    root_child0.insert_child(&mut root_child0_child0, 0);
    root.calculate_layout(Undefined, Undefined, Direction::LTR);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0.get_layout_height());

    assert_eq!(0 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());

    root.calculate_layout(Undefined, Undefined, Direction::RTL);

    assert_eq!(0 as f32, root.get_layout_left());
    assert_eq!(0 as f32, root.get_layout_top());
    assert_eq!(100 as f32, root.get_layout_width());
    assert_eq!(100 as f32, root.get_layout_height());

    assert_eq!(0 as f32, root_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0.get_layout_height());

    assert_eq!(0 as f32, root_child0_child0.get_layout_left());
    assert_eq!(0 as f32, root_child0_child0.get_layout_top());
    assert_eq!(100 as f32, root_child0_child0.get_layout_width());
    assert_eq!(200 as f32, root_child0_child0.get_layout_height());
}
