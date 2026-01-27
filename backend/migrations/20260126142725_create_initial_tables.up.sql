-- TYPE
CREATE TYPE user_role AS ENUM ('Admin', 'Manager', 'Member');
CREATE TYPE node_type AS ENUM ('Initiative', 'Project', 'SubProject', 'Job', 'AdjustmentBuffer');
CREATE TYPE account_type AS ENUM ('Revenue', 'CostOfGoodsSold', 'SellingGeneralAdmin');
CREATE TYPE entry_category AS ENUM ('Plan', 'Result');
CREATE TYPE change_type as ENUM ('Create', 'Update', 'Delete');

-- TABLE
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL DEFAULT '',
    role user_role NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE scenarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID NOT NULL REFERENCES users(id),
    deleted_at TIMESTAMPTZ,
    deleted_by UUID REFERENCES users(id)
);

CREATE TABLE account_items (
    id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    account_type account_type NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);

CREATE TABLE plan_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    scenario_id UUID NOT NULL REFERENCES scenarios(id),
    lineage_id UUID NOT NULL,
    node_type node_type NOT NULL,
    parent_id UUID REFERENCES plan_nodes(id),
    service_id UUID REFERENCES services(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID NOT NULL REFERENCES users(id)
);

CREATE TABLE pl_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    target_month DATE NOT NULL,
    entry_category entry_category NOT NULL,
    node_id UUID NOT NULL REFERENCES plan_nodes(id),
    account_item_id UUID NOT NULL REFERENCES account_items(id),
    amount NUMERIC(20, 4) NOT NULL DEFAULT 0,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID NOT NULL REFERENCES users(id)
);

CREATE TABLE pl_entry_histories(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entry_id UUID NOT NULL REFERENCES pl_entries(id) ON DELETE CASCADE,
    change_type change_type NOT NULL,
    previous_amount NUMERIC(20, 4),
    new_amount NUMERIC(20, 4) NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    changed_by UUID NOT NULL REFERENCES users(id),
    operation_source TEXT
);

-- INDEX
CREATE INDEX idx_plan_nodes_scenario_id ON plan_nodes(scenario_id);
CREATE INDEX idx_plan_nodes_lineage_id ON plan_nodes(lineage_id);
CREATE INDEX idx_plan_nodes_parent_id ON plan_nodes(parent_id);
CREATE INDEX idx_pl_entries_node_id ON pl_entries(node_id);
CREATE INDEX idx_pl_entries_target_month ON pl_entries(target_month);
