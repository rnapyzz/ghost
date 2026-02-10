import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";
import type { CreateScenarioDTO, Scenario, UpdateScenarioDTO } from "@/types";

type RolloverScenarioDTO = {
    name: string;
    start_date: string;
    end_date: string;
};

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

export const useCurrentScenario = () => {
    const { data: scenarios } = useScenarios();
    return scenarios?.find((s: Scenario) => s.is_current) || null;
};

export const useScenarioMutations = () => {
    const queryClient = useQueryClient();

    const createScenario = useMutation({
        mutationFn: (data: CreateScenarioDTO) => api.post("/scenarios", data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: scenarioKeys.all });
        },
    });

    const activateScenario = useMutation({
        mutationFn: (id: string) => api.post(`/scenarios/${id}/activate`),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["scenarios"] });
            queryClient.invalidateQueries({ queryKey: ["planNodes"] });
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

    const rolloverScenario = useMutation({
        mutationFn: ({
            sourceId,
            data,
        }: {
            sourceId: string;
            data: RolloverScenarioDTO;
        }) => api.post(`/scenarios/${sourceId}/rollover`, data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["scenarios"] });
            queryClient.invalidateQueries({ queryKey: ["planNodes"] });
        },
    });

    return {
        createScenario,
        activateScenario,
        updateScenario,
        deleteScenario,
        rolloverScenario,
    };
};
