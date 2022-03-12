table! {
    nodes (device_id) {
        device_id -> Int8,
        hits -> Int4,
        last_hit_at_epoch -> Int8,
        connected -> Bool,
    }
}
