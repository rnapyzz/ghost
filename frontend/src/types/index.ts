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
    deleted_at?: string;
    deleted_by?: string;
};
