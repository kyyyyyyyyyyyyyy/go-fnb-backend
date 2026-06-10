--
-- PostgreSQL database dump
--

\restrict htQtIakf62Ii92AAsNdjdaeoJf0OUozkPOJYTFI9z39sUdmjV77f0svFGblwVWb

-- Dumped from database version 16.14 (Ubuntu 16.14-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.14 (Ubuntu 16.14-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: qr_type; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.qr_type AS ENUM (
    'single',
    'multi'
);


ALTER TYPE public.qr_type OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: addresses; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.addresses (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    address_line text NOT NULL,
    city character varying(100) NOT NULL,
    province character varying(100) NOT NULL,
    postal_code character varying(20),
    latitude double precision,
    longitude double precision,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.addresses OWNER TO postgres;

--
-- Name: categories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.categories (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(100) NOT NULL,
    outlet_id uuid NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.categories OWNER TO postgres;

--
-- Name: invites; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.invites (
    id uuid NOT NULL,
    outlet_id uuid NOT NULL,
    role character varying(20) NOT NULL,
    token text NOT NULL,
    expired_at timestamp without time zone,
    used boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    CONSTRAINT invites_role_check CHECK (((role)::text = ANY ((ARRAY['admin'::character varying, 'cashier'::character varying])::text[])))
);


ALTER TABLE public.invites OWNER TO postgres;

--
-- Name: order_items; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.order_items (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    order_id uuid,
    product_id uuid,
    qty integer NOT NULL,
    price bigint NOT NULL
);


ALTER TABLE public.order_items OWNER TO postgres;

--
-- Name: orders; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.orders (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    outlet_id uuid,
    table_id uuid,
    status character varying(50) DEFAULT 'pending'::character varying,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.orders OWNER TO postgres;

--
-- Name: outlets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.outlets (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(150) NOT NULL,
    owner_id uuid,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    address_id uuid NOT NULL
);


ALTER TABLE public.outlets OWNER TO postgres;

--
-- Name: product_categories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.product_categories (
    product_id uuid NOT NULL,
    category_id uuid NOT NULL
);


ALTER TABLE public.product_categories OWNER TO postgres;

--
-- Name: products; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.products (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(150) NOT NULL,
    price bigint NOT NULL,
    outlet_id uuid,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.products OWNER TO postgres;

--
-- Name: qr_code_tables; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.qr_code_tables (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    qr_id uuid NOT NULL,
    table_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.qr_code_tables OWNER TO postgres;

--
-- Name: qr_codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.qr_codes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    outlet_id uuid NOT NULL,
    slug text NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    expired_at timestamp without time zone
);


ALTER TABLE public.qr_codes OWNER TO postgres;

--
-- Name: tables; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tables (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(50) NOT NULL,
    outlet_id uuid,
    status character varying(50) DEFAULT 'available'::character varying,
    token text,
    location character varying(50),
    created_at timestamp without time zone DEFAULT now()
);


ALTER TABLE public.tables OWNER TO postgres;

--
-- Name: transactions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.transactions (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    order_id uuid,
    payment_method character varying(50),
    status character varying(50) DEFAULT 'pending'::character varying,
    amount bigint NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.transactions OWNER TO postgres;

--
-- Name: user_outlets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_outlets (
    user_id uuid NOT NULL,
    outlet_id uuid NOT NULL,
    role character varying(50) NOT NULL
);


ALTER TABLE public.user_outlets OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(100) NOT NULL,
    email character varying(150) NOT NULL,
    password text,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    provider text DEFAULT 'local'::text,
    provider_id text
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Data for Name: addresses; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.addresses (id, address_line, city, province, postal_code, latitude, longitude, created_at) FROM stdin;
25d7921b-919d-47ef-a33a-23ecda13caf5	j;.Pramuka No 81, Puwawianagun Kuningan	Kuningan	jawa barat	4551	\N	\N	2026-04-20 15:28:32.541142
bb0c54fb-353f-4dd7-9966-43143ea66044	j;.Pramuka No 81, Puwawianagun Kuningan	Kuningan	jawa barat	4551	6.877404156793134	108.49747828279568	2026-04-20 15:29:00.541549
6e030dac-05b1-4ed5-a2be-cd58d195bedc	j;.Pramuka No 81, Puwawianagun Kuningan	Kuningan	jawa barat	4551	6.877404156793134	108.49747828279568	2026-06-05 03:27:42.643268
3169ac15-36f9-4a3f-8a17-a25c3461d700	jl pramuki	kuningan	jabar	45551	22	11	2026-06-05 12:01:10.251779
bead10a5-2a74-4c73-a5b1-09f3397bd4a3	jl pramuka	juningan	jawa barat	45556	236	236	2026-06-05 12:58:55.18209
\.


--
-- Data for Name: categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.categories (id, name, outlet_id, created_at) FROM stdin;
76eddabd-caed-474b-ac23-6a47f9885ac9	mojito	2eff40b1-5971-4ad3-8e36-8f485c0403a1	2026-06-10 21:52:56.744103
\.


--
-- Data for Name: invites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.invites (id, outlet_id, role, token, expired_at, used, created_at) FROM stdin;
77dfeee4-de9e-4639-9e63-0123bb6e2434	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier	ecd0b48f-fe1d-4794-a309-0ed0dd24b40a	2026-06-06 03:42:35.670674	t	2026-06-05 03:42:35.670674
d9eb06d1-00e5-4fa2-a884-5497ff322b1a	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier	2d4a29cd-3a82-459a-85dc-b427ee186f89	2026-06-06 04:03:06.555408	f	2026-06-05 04:03:06.555408
25d0a686-e3ea-4ef3-93ca-c6056e461f66	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier	8468b9b4-3519-48d7-997b-5d005749aba3	2026-06-06 04:03:49.542567	f	2026-06-05 04:03:49.542567
a7e751f6-ca8a-4e42-ae77-b01badc25218	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier	edbfdd02-ab82-4956-81ba-57b71f2f1180	2026-06-06 11:08:39.992482	t	2026-06-05 11:08:39.992482
71121d45-5178-42a1-a6be-8f407bdeb31f	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier	57b51c14-0b35-48cd-9f3b-9959b9e94c18	2026-06-06 11:44:07.282971	f	2026-06-05 11:44:07.282971
35e5b979-e9ee-4f31-a0d9-f8fd9dc79023	47a6d11e-a00a-4e16-8096-5d2aa065f931	cashier	c0004cc7-94c8-40f5-b64b-81e0858bbdd7	2026-06-06 12:03:00.488979	f	2026-06-05 12:03:00.488979
babe0a37-a3aa-4a7b-8dfd-8d0a7c38c999	d59a4150-063e-4df0-af5c-635e3f54673c	cashier	cd346fe8-1c7e-4688-a25f-a24e0f4679fd	2026-06-06 12:59:39.052989	f	2026-06-05 12:59:39.052989
\.


--
-- Data for Name: order_items; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.order_items (id, order_id, product_id, qty, price) FROM stdin;
\.


--
-- Data for Name: orders; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.orders (id, outlet_id, table_id, status, created_at) FROM stdin;
\.


--
-- Data for Name: outlets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.outlets (id, name, owner_id, created_at, address_id) FROM stdin;
15c4bdc8-984d-4b8f-85bc-8dada8265586	semoga jadi	75a038e2-a188-4d55-8b65-149742dabf1c	2026-04-20 15:28:32.541142	25d7921b-919d-47ef-a33a-23ecda13caf5
2743702f-5a05-40bb-aea0-6a85edebc809	tengxiii	75a038e2-a188-4d55-8b65-149742dabf1c	2026-04-20 15:29:00.541549	bb0c54fb-353f-4dd7-9966-43143ea66044
2eff40b1-5971-4ad3-8e36-8f485c0403a1	for real	151ed1f3-f433-4731-aab9-242b612b7179	2026-06-05 03:27:42.643268	6e030dac-05b1-4ed5-a2be-cd58d195bedc
47a6d11e-a00a-4e16-8096-5d2aa065f931	sisi jalak	632fa922-f4c6-4c5d-ab16-8b49eb1b8cb9	2026-06-05 12:01:10.251779	3169ac15-36f9-4a3f-8a17-a25c3461d700
d59a4150-063e-4df0-af5c-635e3f54673c	ngopss	151ed1f3-f433-4731-aab9-242b612b7179	2026-06-05 12:58:55.18209	bead10a5-2a74-4c73-a5b1-09f3397bd4a3
\.


--
-- Data for Name: product_categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.product_categories (product_id, category_id) FROM stdin;
\.


--
-- Data for Name: products; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.products (id, name, price, outlet_id, created_at) FROM stdin;
\.


--
-- Data for Name: qr_code_tables; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qr_code_tables (id, qr_id, table_id, created_at) FROM stdin;
796b334d-7474-4513-a884-1ffb9d547077	c803e04c-37d9-43b2-934e-84b25711e93b	b44c431d-2c8c-4b2d-94ff-cbe9a4b8e7a1	2026-05-28 23:22:38.240566+07
54916c91-70b0-4f10-a237-24fc4a1a65d0	c803e04c-37d9-43b2-934e-84b25711e93b	39fb1cd6-a32e-4146-b2a1-00125c1d4904	2026-05-28 23:22:38.246227+07
2d17a690-0ed2-4fc9-aeaf-fc5c2ff5abfb	d6e0d6e2-906b-4471-bbf3-3579d884e00d	9d4e7ac3-9b39-4162-a4db-cdfdbf448d45	2026-05-28 23:23:46.240773+07
004e441b-ea98-4d47-9e85-97eadf4d25e8	dd045df5-63f3-4144-9ed3-710c7d26459b	9d4e7ac3-9b39-4162-a4db-cdfdbf448d45	2026-06-04 00:21:51.137437+07
05078dfe-729f-498d-b868-8ef9b82e4e5f	895fc917-6362-485d-8692-24dc1617bce4	9d4e7ac3-9b39-4162-a4db-cdfdbf448d45	2026-06-04 09:10:26.828408+07
83d1ffbd-7186-4b19-9784-117f0200672d	ca85a263-97bd-4e26-8f99-51af72765b6c	9d4e7ac3-9b39-4162-a4db-cdfdbf448d45	2026-06-04 09:25:14.314446+07
\.


--
-- Data for Name: qr_codes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qr_codes (id, outlet_id, slug, created_at, expired_at) FROM stdin;
c803e04c-37d9-43b2-934e-84b25711e93b	15c4bdc8-984d-4b8f-85bc-8dada8265586	PQKAfcAuhFTX	2026-05-28 23:22:38.234687+07	\N
d6e0d6e2-906b-4471-bbf3-3579d884e00d	15c4bdc8-984d-4b8f-85bc-8dada8265586	CXyHuP8w2EfY	2026-05-28 23:23:46.234419+07	\N
dd045df5-63f3-4144-9ed3-710c7d26459b	15c4bdc8-984d-4b8f-85bc-8dada8265586	aNPYgPnLR5Uv	2026-06-04 00:21:51.04956+07	\N
895fc917-6362-485d-8692-24dc1617bce4	15c4bdc8-984d-4b8f-85bc-8dada8265586	8hyJN52o4BP8	2026-06-04 09:10:26.781069+07	\N
ca85a263-97bd-4e26-8f99-51af72765b6c	15c4bdc8-984d-4b8f-85bc-8dada8265586	dSegd6ouGMhZ	2026-06-04 09:25:14.310292+07	\N
\.


--
-- Data for Name: tables; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tables (id, name, outlet_id, status, token, location, created_at) FROM stdin;
b44c431d-2c8c-4b2d-94ff-cbe9a4b8e7a1	meja 12	15c4bdc8-984d-4b8f-85bc-8dada8265586	occupied	AqiwSCUaEK0hqBUciACBEQnwwSSwtHL7	didie	2026-05-28 15:53:04.439529
39fb1cd6-a32e-4146-b2a1-00125c1d4904	meja 10	15c4bdc8-984d-4b8f-85bc-8dada8265586	available	\N	dditu	2026-05-05 16:39:45.415719
9d4e7ac3-9b39-4162-a4db-cdfdbf448d45	pecut	15c4bdc8-984d-4b8f-85bc-8dada8265586	available	\N	ammart	2026-05-28 16:23:29.999154
a1a2a402-57a9-4eb6-8661-5bd04dd125a2	12	2eff40b1-5971-4ad3-8e36-8f485c0403a1	available	\N	fidie	2026-06-05 12:01:55.356404
7e2b654d-6b79-4521-842a-b6ffe567d6fc	2	2eff40b1-5971-4ad3-8e36-8f485c0403a1	available	\N	dibawah	2026-06-05 13:00:33.931936
\.


--
-- Data for Name: transactions; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.transactions (id, order_id, payment_method, status, amount, created_at) FROM stdin;
\.


--
-- Data for Name: user_outlets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_outlets (user_id, outlet_id, role) FROM stdin;
75a038e2-a188-4d55-8b65-149742dabf1c	15c4bdc8-984d-4b8f-85bc-8dada8265586	owner
75a038e2-a188-4d55-8b65-149742dabf1c	2743702f-5a05-40bb-aea0-6a85edebc809	owner
151ed1f3-f433-4731-aab9-242b612b7179	2eff40b1-5971-4ad3-8e36-8f485c0403a1	owner
56ad8b5d-3fde-4c42-85bc-a0c42c13898f	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier
b220a69d-e17b-4fc4-9eae-377ae6914b23	2eff40b1-5971-4ad3-8e36-8f485c0403a1	cashier
632fa922-f4c6-4c5d-ab16-8b49eb1b8cb9	47a6d11e-a00a-4e16-8096-5d2aa065f931	owner
151ed1f3-f433-4731-aab9-242b612b7179	d59a4150-063e-4df0-af5c-635e3f54673c	owner
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, email, password, created_at, provider, provider_id) FROM stdin;
2c7eabf9-63a9-4b08-b822-3be8611c90ce	kayu	admin@gmail.com	$2b$12$/pB/TaJSm4JzC5tQtVSm6eGc4ncCeDEC1LAZig6k19eMcN.QX53mm	2026-04-11 15:00:05.874609	\N	\N
151ed1f3-f433-4731-aab9-242b612b7179	tengsi	coba1@gmail.com	$2b$12$22mZB9rOgMA0VMBONAYx7OzZ3.CQgsGEvWmVI3ADfDfYPZzF3XfKa	2026-04-14 01:44:52.409065	\N	\N
75a038e2-a188-4d55-8b65-149742dabf1c	Muhammad Rizky	20240810023@uniku.ac.id	\N	2026-04-19 17:00:17.403292	google	117894576446598891641
56ad8b5d-3fde-4c42-85bc-a0c42c13898f	can you	tengsi@uniku.ac.usa	$2b$12$ATXv26py2x9Clklg9NQlnOm1LxkSuAdJ7x7I3SlZrBpRCIw1CJcHy	2026-06-05 03:52:51.128403	local	\N
b220a69d-e17b-4fc4-9eae-377ae6914b23	kucel	tengsi@uniku.ac.ea	$2b$12$kz4x5CrsY7l2f.h9CwKQ7.0Fy9CcOtTY9w900iWZMWBkYVKbelpFe	2026-06-05 11:17:32.846581	local	\N
632fa922-f4c6-4c5d-ab16-8b49eb1b8cb9	anonimus	kayhu@pride.id	$2b$12$5Z5gAy1aDZSZQLGiTVx3sOV/HdJDdKjMzuqaWr5tmyK9yOruOl1pq	2026-06-05 11:56:50.971587	local	\N
\.


--
-- Name: addresses addresses_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.addresses
    ADD CONSTRAINT addresses_pkey PRIMARY KEY (id);


--
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (id);


--
-- Name: users email_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT email_unique UNIQUE (email);


--
-- Name: invites invites_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_pkey PRIMARY KEY (id);


--
-- Name: invites invites_token_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_token_key UNIQUE (token);


--
-- Name: order_items order_items_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_pkey PRIMARY KEY (id);


--
-- Name: orders orders_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_pkey PRIMARY KEY (id);


--
-- Name: outlets outlets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT outlets_pkey PRIMARY KEY (id);


--
-- Name: product_categories product_categories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_pkey PRIMARY KEY (product_id, category_id);


--
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (id);


--
-- Name: qr_code_tables qr_code_tables_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_pkey PRIMARY KEY (id);


--
-- Name: qr_codes qr_codes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT qr_codes_pkey PRIMARY KEY (id);


--
-- Name: qr_codes qr_codes_slug_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT qr_codes_slug_key UNIQUE (slug);


--
-- Name: tables tables_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_pkey PRIMARY KEY (id);


--
-- Name: tables tables_token_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_token_unique UNIQUE (token);


--
-- Name: transactions transactions_order_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_order_id_key UNIQUE (order_id);


--
-- Name: transactions transactions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);


--
-- Name: outlets unique_outlet_address; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT unique_outlet_address UNIQUE (address_id);


--
-- Name: qr_code_tables unique_qr_table; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT unique_qr_table UNIQUE (qr_id, table_id);


--
-- Name: qr_codes unique_slug; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT unique_slug UNIQUE (slug);


--
-- Name: user_outlets user_outlets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_pkey PRIMARY KEY (user_id, outlet_id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_provider_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_provider_id_key UNIQUE (provider_id);


--
-- Name: idx_addresses_lat_lng; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_lat_lng ON public.addresses USING btree (latitude, longitude);


--
-- Name: idx_addresses_province; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_province ON public.addresses USING btree (province);


--
-- Name: idx_invites_token; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_invites_token ON public.invites USING btree (token);


--
-- Name: idx_order_items_order; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_order_items_order ON public.order_items USING btree (order_id);


--
-- Name: idx_orders_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_orders_outlet ON public.orders USING btree (outlet_id);


--
-- Name: idx_products_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_products_outlet ON public.products USING btree (outlet_id);


--
-- Name: idx_qr_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_id ON public.qr_code_tables USING btree (qr_id);


--
-- Name: idx_qr_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_outlet ON public.qr_codes USING btree (outlet_id);


--
-- Name: idx_qr_slug; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_slug ON public.qr_codes USING btree (slug);


--
-- Name: idx_table_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_table_id ON public.qr_code_tables USING btree (table_id);


--
-- Name: idx_tables_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tables_outlet ON public.tables USING btree (outlet_id);


--
-- Name: categories categories_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: outlets fk_outlets_address; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT fk_outlets_address FOREIGN KEY (address_id) REFERENCES public.addresses(id) ON DELETE CASCADE;


--
-- Name: qr_code_tables fk_qr; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT fk_qr FOREIGN KEY (qr_id) REFERENCES public.qr_codes(id) ON DELETE CASCADE;


--
-- Name: qr_codes fk_qr_outlet; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT fk_qr_outlet FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: qr_code_tables fk_table; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT fk_table FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE CASCADE;


--
-- Name: invites invites_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: order_items order_items_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id) ON DELETE CASCADE;


--
-- Name: order_items order_items_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id) ON DELETE SET NULL;


--
-- Name: orders orders_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: orders orders_table_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_table_id_fkey FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE SET NULL;


--
-- Name: outlets outlets_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT outlets_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: product_categories product_categories_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_category_id_fkey FOREIGN KEY (category_id) REFERENCES public.categories(id) ON DELETE CASCADE;


--
-- Name: product_categories product_categories_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id) ON DELETE CASCADE;


--
-- Name: products products_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: qr_code_tables qr_code_tables_qr_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_qr_id_fkey FOREIGN KEY (qr_id) REFERENCES public.qr_codes(id) ON DELETE CASCADE;


--
-- Name: qr_code_tables qr_code_tables_table_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_table_id_fkey FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE CASCADE;


--
-- Name: tables tables_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: transactions transactions_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id) ON DELETE CASCADE;


--
-- Name: user_outlets user_outlets_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- Name: user_outlets user_outlets_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict htQtIakf62Ii92AAsNdjdaeoJf0OUozkPOJYTFI9z39sUdmjV77f0svFGblwVWb

