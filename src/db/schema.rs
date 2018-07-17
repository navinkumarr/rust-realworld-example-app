table! {
    articles (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        slug -> Varchar,
        title -> Varchar,
        description -> Varchar,
        body -> Text,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

table! {
    article_tag (article_id, tag_id) {
        article_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
    }
}

table! {
    comments (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        article_id -> Unsigned<Integer>,
        body -> Text,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

table! {
    favorites (user_id, article_id) {
        user_id -> Unsigned<Integer>,
        article_id -> Unsigned<Integer>,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

table! {
    follows (follower_id, followed_id) {
        follower_id -> Unsigned<Integer>,
        followed_id -> Unsigned<Integer>,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Varchar,
        token -> Varchar,
        created_at -> Unsigned<Bigint>,
        updated_at -> Unsigned<Bigint>,
    }
}

joinable!(article_tag -> articles (article_id));
joinable!(article_tag -> tags (tag_id));
joinable!(articles -> users (user_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorites -> articles (article_id));
joinable!(favorites -> users (user_id));

allow_tables_to_appear_in_same_query!(
    articles,
    article_tag,
    comments,
    favorites,
    follows,
    tags,
    users,
);
