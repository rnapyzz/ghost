import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api.ts";

export const nodesKeys = {
    all: ["planNodes"] as const,
    lists: () => [...nodesKeys.all, "list"] as const,
};

export const usePlanNodes = () => {
    return useQuery({
        queryKey: nodesKeys.all,
        queryFn: async () => {
            const res = await api.get("/plan-nodes");
            return res.data;
        },
    });
};
