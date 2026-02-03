import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";
import type { CreateServiceDTO } from "@/types";

export const serviceKeys = {
    all: ["services"] as const,
};

export const useServices = () => {
    return useQuery({
        queryKey: serviceKeys.all,
        queryFn: async () => {
            const res = await api.get("/services");
            return res.data;
        },
    });
};

export const useServiceMutations = () => {
    const queryClient = useQueryClient();

    const createService = useMutation({
        mutationFn: (data: CreateServiceDTO) => api.post("/services", data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: serviceKeys.all });
        },
    });

    return { createService };
};
