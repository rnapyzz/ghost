import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";
import type { CreateScenarioDTO, UpdateScenarioDTO } from "@/types";

export const scenarioKeys = {
    all: ["scenarios"] as const,
};

export const useScenarios = () => {
    return useQuery({
        queryKey: scenarioKeys.all,
        queryFn: async () => {
            const res = await api.get("/scenarios");
            return res.data;
        },
    });
};

export const useScenarioMutations = () => {
    const queryClient = useQueryClient();

    const createScenario = useMutation({
        mutationFn: (data: CreateScenarioDTO) => api.post("/scenarios", data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: scenarioKeys.all });
        },
    });

    const updateScenario = useMutation({
        mutationFn: ({ id, data }: { id: string; data: UpdateScenarioDTO }) =>
            api.patch(`/scenarios/${id}`, data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: scenarioKeys.all });
        },
    });

    const deleteScenario = useMutation({
        mutationFn: (id: string) => api.delete(`/scenarios/${id}`),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: scenarioKeys.all });
        },
    });

    return { createScenario, updateScenario, deleteScenario };
};
