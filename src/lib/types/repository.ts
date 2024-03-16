export type Branch = {
    name: string;
    tip_commit_id: string;
};

export type Repository = {
    branches: Branch[];
};
