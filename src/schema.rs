table! {
    wp_posts (ID) {
        ID -> Unsigned<BigInt>,
        post_author -> Unsigned<BigInt>,
        //post_date -> Timestamp,
        //post_date_gmt -> Timestamp,
        post_content -> Text,
        post_title -> Text,
        post_excerpt -> Text,
        post_status -> VarChar,
        comment_status -> VarChar,
        ping_status -> VarChar,
        post_password -> VarChar,
        post_name -> VarChar,
        to_ping -> Text,
        pinged -> Text,
        //post_modified -> Timestamp,
        //post_modified_gmt -> Timestamp,
        post_content_filtered -> VarChar,
        post_parent -> Unsigned<BigInt>,
        guid -> VarChar,
        menu_order -> Integer,
        post_type -> VarChar,
        post_mime_type -> VarChar,
        comment_count -> BigInt,
    }
}