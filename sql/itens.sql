-- Tabela de itens para Supabase/Postgres

create table if not exists public.itens (
    id serial primary key,
    nome text not null,
    preco double precision not null
);

-- Permissões básicas (ajuste conforme sua política de segurança)
-- Para permitir que a role anon (chave pública) acesse diretamente:
grant select, insert, update, delete on table public.itens to anon;
grant usage, select on sequence public.itens_id_seq to anon;
