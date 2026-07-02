use chrono::{TimeZone, Utc};
use ocel_core::{
    AttrType, AttrValue, AttributeDefinition, Event, EventAttribute, EventType, Object,
    ObjectAttribute, ObjectType, Ocel, OcelBuilder, Relationship, Violation,
};

fn sample() -> Ocel {
    let mut b = OcelBuilder::new();
    b.add_event_type(EventType {
        name: "create order".into(),
        attributes: vec![AttributeDefinition {
            name: "count".into(),
            value_type: AttrType::Integer,
        }],
    });
    b.add_object_type(ObjectType {
        name: "order".into(),
        attributes: vec![AttributeDefinition {
            name: "price".into(),
            value_type: AttrType::Float,
        }],
    });
    b.add_object(Object {
        id: "o1".into(),
        object_type: "order".into(),
        attributes: vec![
            ObjectAttribute {
                name: "price".into(),
                value: AttrValue::Float(10.0),
                time: Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap(),
            },
            ObjectAttribute {
                name: "price".into(),
                value: AttrValue::Float(20.0),
                time: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
            },
        ],
        relationships: vec![],
    });
    b.add_event(Event {
        id: "e1".into(),
        event_type: "create order".into(),
        time: Utc.with_ymd_and_hms(2022, 1, 2, 0, 0, 0).unwrap(),
        attributes: vec![EventAttribute {
            name: "count".into(),
            value: AttrValue::Integer(1),
        }],
        relationships: vec![Relationship {
            object_id: "o1".into(),
            qualifier: "created".into(),
        }],
    });
    b.build().unwrap()
}

#[test]
fn builds_valid_log() {
    let ocel = sample();
    assert_eq!(ocel.events.len(), 1);
    assert_eq!(ocel.objects.len(), 1);
}

#[test]
fn detects_dangling_e2o() {
    let mut b = OcelBuilder::new();
    b.add_event_type(EventType {
        name: "e".into(),
        attributes: vec![],
    });
    b.add_event(Event {
        id: "e1".into(),
        event_type: "e".into(),
        time: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
        attributes: vec![],
        relationships: vec![Relationship {
            object_id: "missing".into(),
            qualifier: "q".into(),
        }],
    });
    assert_eq!(
        b.build().unwrap_err(),
        vec![Violation::DanglingE2O {
            event: "e1".into(),
            object: "missing".into(),
        }]
    );
}

#[test]
fn detects_duplicate_object_id() {
    let mut b = OcelBuilder::new();
    b.add_object_type(ObjectType {
        name: "order".into(),
        attributes: vec![],
    });
    let make = || Object {
        id: "o1".into(),
        object_type: "order".into(),
        attributes: vec![],
        relationships: vec![],
    };
    b.add_object(make());
    b.add_object(make());
    assert_eq!(
        b.build().unwrap_err(),
        vec![Violation::DuplicateObjectId("o1".into())]
    );
}

#[test]
fn detects_undeclared_event_type() {
    let mut b = OcelBuilder::new();
    b.add_event(Event {
        id: "e1".into(),
        event_type: "ghost".into(),
        time: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
        attributes: vec![],
        relationships: vec![],
    });
    assert_eq!(
        b.build().unwrap_err(),
        vec![Violation::UndeclaredEventType {
            event: "e1".into(),
            event_type: "ghost".into(),
        }]
    );
}

#[test]
fn forward_fill_attribute_at() {
    let ocel = sample();
    let order = &ocel.objects[0];
    let early = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
    let late = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let before_any = Utc.with_ymd_and_hms(1960, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(
        order.attribute_at("price", early),
        Some(&AttrValue::Float(10.0))
    );
    assert_eq!(
        order.attribute_at("price", late),
        Some(&AttrValue::Float(20.0))
    );
    assert_eq!(order.attribute_at("price", before_any), None);
    assert_eq!(order.attribute_at("missing", late), None);
}

#[test]
fn serde_round_trip() {
    let ocel = sample();
    let json = serde_json::to_string(&ocel).unwrap();
    let back: Ocel = serde_json::from_str(&json).unwrap();
    assert_eq!(ocel, back);
}

#[test]
fn e2o_flattened() {
    let ocel = sample();
    let rels: Vec<_> = ocel.e2o().collect();
    assert_eq!(rels.len(), 1);
    assert_eq!(rels[0].event_id, "e1");
    assert_eq!(rels[0].object_id, "o1");
    assert_eq!(rels[0].qualifier, "created");
}

#[test]
fn event_columns_view() {
    let ocel = sample();
    let cols = ocel.event_columns();
    assert_eq!(cols.ids, vec!["e1"]);
    assert_eq!(cols.types, vec!["create order"]);
    assert_eq!(cols.times.len(), 1);
}
