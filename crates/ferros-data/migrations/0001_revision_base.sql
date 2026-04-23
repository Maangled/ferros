create table if not exists revision_base (
    revision_id bigserial primary key,
    lineage_id uuid not null,
    entity_id uuid not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create table if not exists entity_snapshot (
    snapshot_id bigserial primary key,
    revision_id bigint not null references revision_base(revision_id) on delete cascade,
    snapshot jsonb not null,
    created_at timestamptz not null default now(),
    check (jsonb_typeof(snapshot) = 'object')
);

create table if not exists ordered_child (
    ordered_child_id bigserial primary key,
    revision_id bigint not null references revision_base(revision_id) on delete cascade,
    parent_card_id uuid,
    parent_deck_id uuid,
    sort_key bigint not null,
    created_at timestamptz not null default now(),
    check (parent_card_id is not null or parent_deck_id is not null)
);
