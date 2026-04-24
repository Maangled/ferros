alter table ordered_child
    add constraint ordered_child_single_parent_scope
    check (
        not (
            parent_card_id is not null
            and parent_deck_id is not null
        )
    );