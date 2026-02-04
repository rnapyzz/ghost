export type User = {
    id: string;
    name: string;
    email: string;
};

export type ApiResponse<T> = {
    data: T;
    message?: string;
};

export type PlanNode = {
    id: string;
    scenario_id: string;
    parent_id?: string;
    lineage_id: string;
    title: string;
    description?: string;
    node_type: string;
    display_order: number;
    service_id?: string;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
};

export type CreatePlanNodeDTO = {
    scenario_id: string;
    parent_id?: string;
    lineage_id?: string;
    title: string;
    description?: string;
    node_type: string;
    display_order: number;
    service_id?: string;
};

export type Scenario = {
    id: string;
    name: string;
    description?: string;
    start_date: string; // YYYY-MM-DD
    end_date: string; // YYYY-MM-DD
    is_locked: boolean;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
};

export type CreateScenarioDTO = {
    name: string;
    description?: string;
    start_date: string;
    end_date: string;
};

export type UpdateScenarioDTO = Partial<CreateScenarioDTO> & {
    is_locked?: boolean;
};

export type Service = {
    id: string;
    name: string;
    slug: string;
    display_order: number;
    created_at: string;
    updated_at: string;
};

export type CreateServiceDTO = {
    name: string;
    slug: string;
    display_order: number;
};
