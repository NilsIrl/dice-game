table! {
    /// Representation of the `games` table.
    ///
    /// (Automatically generated by Diesel.)
    games (id) {
        /// The `id` column of the `games` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `player1_id` column of the `games` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        player1_id -> Int4,
        /// The `player2_id` column of the `games` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        player2_id -> Nullable<Int4>,
        /// The `player1_score` column of the `games` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        player1_score -> Int2,
        /// The `player2_score` column of the `games` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        player2_score -> Int2,
        /// The `player1_extra` column of the `games` table.
        ///
        /// Its SQL type is `Array<Int2>`.
        ///
        /// (Automatically generated by Diesel.)
        player1_extra -> Array<Int2>,
        /// The `player2_extra` column of the `games` table.
        ///
        /// Its SQL type is `Array<Int2>`.
        ///
        /// (Automatically generated by Diesel.)
        player2_extra -> Array<Int2>,
    }
}

table! {
    /// Representation of the `rounds` table.
    ///
    /// (Automatically generated by Diesel.)
    rounds (id) {
        /// The `id` column of the `rounds` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `game_id` column of the `rounds` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        game_id -> Int4,
        /// The `round_count` column of the `rounds` table.
        ///
        /// Its SQL type is `Int2`.
        ///
        /// (Automatically generated by Diesel.)
        round_count -> Int2,
        /// The `player1_throws` column of the `rounds` table.
        ///
        /// Its SQL type is `Array<Int2>`.
        ///
        /// (Automatically generated by Diesel.)
        player1_throws -> Array<Int2>,
        /// The `player2_throws` column of the `rounds` table.
        ///
        /// Its SQL type is `Array<Int2>`.
        ///
        /// (Automatically generated by Diesel.)
        player2_throws -> Array<Int2>,
        /// The `player1_played` column of the `rounds` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        player1_played -> Bool,
        /// The `player2_played` column of the `rounds` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        player2_played -> Bool,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `username` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Text,
        /// The `password` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        password -> Text,
        /// The `score` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        score -> Int4,
    }
}

joinable!(rounds -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    rounds,
    users,
);