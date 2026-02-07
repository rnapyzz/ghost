import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";
import type { CreatePlanNodeDTO } from "@/types";

export const planNodesKeys = {
    all: ["planNodes"] as const,
    lists: () => [...planNodesKeys.all, "list"] as const,
    byScenario: (scenarioId: string | undefined) =>
        [...planNodesKeys.all, { scenarioId }] as const,
};

export const usePlanNodes = (scenarioId?: string) => {
    return useQuery({
        queryKey: planNodesKeys.byScenario(scenarioId),
        queryFn: async () => {
            if (!scenarioId) return [];

            const res = await api.get("/plan-nodes", {
                params: { scenario_id: scenarioId },
            });
            return res.data;
        },
        enabled: !!scenarioId,
    });
};

export const usePlanNodeMutations = () => {
    const queryClient = useQueryClient();

    const createPlanNode = useMutation({
        mutationFn: (data: CreatePlanNodeDTO) => api.post("/plan-nodes", data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: planNodesKeys.all });
        },
    });

    const updatePlanNode = useMutation({
        mutationFn: ({
            id,
            data,
        }: {
            id: string;
            data: Partial<CreatePlanNodeDTO>;
        }) => api.patch(`/plan-nodes/${id}`, data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: planNodesKeys.all });
        },
    });

    const deletePlanNode = useMutation({
        mutationFn: (id: string) => api.delete(`/plan-nodes/${id}`),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: planNodesKeys.all });
        },
    });

    return { createPlanNode, updatePlanNode, deletePlanNode };
};
