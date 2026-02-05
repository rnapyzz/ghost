import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";
import type { CreatePlanNodeDTO } from "@/types";

export const planNodesKeys = {
    all: ["planNodes"] as const,
    lists: () => [...planNodesKeys.all, "list"] as const,
};

export const usePlanNodes = () => {
    return useQuery({
        queryKey: planNodesKeys.all,
        queryFn: async () => {
            const res = await api.get("/plan-nodes");
            return res.data;
        },
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

    return { createPlanNode };
};
