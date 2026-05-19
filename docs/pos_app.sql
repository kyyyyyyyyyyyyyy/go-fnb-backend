--
-- PostgreSQL database dump
--

\restrict PQbuxCBbndQ5If6xdbJvHkJKnjWDPci8nGLFqtbdqoda4MnCjmyouC73XPlhnSH

-- Dumped from database version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)

-- Started on 2026-05-19 22:40:45 WIB

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
-- TOC entry 2 (class 3079 OID 16516)
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- TOC entry 3617 (class 0 OID 0)
-- Dependencies: 2
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- TOC entry 903 (class 1247 OID 16762)
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
-- TOC entry 226 (class 1259 OID 16676)
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
-- TOC entry 219 (class 1259 OID 16565)
-- Name: categories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.categories (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name character varying(100) NOT NULL,
    outlet_id uuid
);


ALTER TABLE public.categories OWNER TO postgres;

--
-- TOC entry 229 (class 1259 OID 16790)
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
-- TOC entry 224 (class 1259 OID 16639)
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
-- TOC entry 223 (class 1259 OID 16621)
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
-- TOC entry 217 (class 1259 OID 16538)
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
-- TOC entry 221 (class 1259 OID 16588)
-- Name: product_categories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.product_categories (
    product_id uuid NOT NULL,
    category_id uuid NOT NULL
);


ALTER TABLE public.product_categories OWNER TO postgres;

--
-- TOC entry 220 (class 1259 OID 16576)
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
-- TOC entry 228 (class 1259 OID 16740)
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
-- TOC entry 227 (class 1259 OID 16706)
-- Name: qr_codes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.qr_codes (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    outlet_id uuid NOT NULL,
    slug text NOT NULL,
    created_at timestamp without time zone DEFAULT now(),
    expired_at timestamp without time zone
);


ALTER TABLE public.qr_codes OWNER TO postgres;

--
-- TOC entry 222 (class 1259 OID 16603)
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
-- TOC entry 225 (class 1259 OID 16655)
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
-- TOC entry 218 (class 1259 OID 16550)
-- Name: user_outlets; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_outlets (
    user_id uuid NOT NULL,
    outlet_id uuid NOT NULL,
    role character varying(50) NOT NULL
);


ALTER TABLE public.user_outlets OWNER TO postgres;

--
-- TOC entry 216 (class 1259 OID 16527)
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
-- TOC entry 3608 (class 0 OID 16676)
-- Dependencies: 226
-- Data for Name: addresses; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.addresses (id, address_line, city, province, postal_code, latitude, longitude, created_at) FROM stdin;
25d7921b-919d-47ef-a33a-23ecda13caf5	j;.Pramuka No 81, Puwawianagun Kuningan	Kuningan	jawa barat	4551	\N	\N	2026-04-20 15:28:32.541142
bb0c54fb-353f-4dd7-9966-43143ea66044	j;.Pramuka No 81, Puwawianagun Kuningan	Kuningan	jawa barat	4551	6.877404156793134	108.49747828279568	2026-04-20 15:29:00.541549
\.


--
-- TOC entry 3601 (class 0 OID 16565)
-- Dependencies: 219
-- Data for Name: categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.categories (id, name, outlet_id) FROM stdin;
\.


--
-- TOC entry 3611 (class 0 OID 16790)
-- Dependencies: 229
-- Data for Name: invites; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.invites (id, outlet_id, role, token, expired_at, used, created_at) FROM stdin;
e1e23447-4991-476c-bf4c-dd917ed3a363	15c4bdc8-984d-4b8f-85bc-8dada8265586	cashier	3bc04048-74a9-4e9b-9b38-fb7058e9c2db	2026-05-04 15:56:24.700123	f	2026-05-03 15:56:24.700123
a220d12d-18ca-4fc3-b55f-72d6492d32cf	15c4bdc8-984d-4b8f-85bc-8dada8265586	cashier	94fcd9ac-1656-4044-bf8f-2330cad02dfb	2026-05-04 16:10:04.331933	f	2026-05-03 16:10:04.331933
b52350f6-89e3-4443-adbe-e050fb039928	15c4bdc8-984d-4b8f-85bc-8dada8265586	cashier	44c1d35b-ac17-45a3-9e35-e44416531b42	2026-05-04 16:16:36.161081	f	2026-05-03 16:16:36.161081
f1214fee-faaf-4330-b66c-cb3d99a80103	15c4bdc8-984d-4b8f-85bc-8dada8265586	cashier	f07ff12b-5384-4d38-90ca-78b9ff24c88f	2026-05-04 16:16:50.896243	t	2026-05-03 16:16:50.896243
c98fc448-8b3a-482a-b3de-c32593a0889d	15c4bdc8-984d-4b8f-85bc-8dada8265586	cashier	729c2e0b-b2ce-41c4-8799-60b35c0b98a3	2026-05-04 16:55:29.871158	f	2026-05-03 16:55:29.871158
\.


--
-- TOC entry 3606 (class 0 OID 16639)
-- Dependencies: 224
-- Data for Name: order_items; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.order_items (id, order_id, product_id, qty, price) FROM stdin;
\.


--
-- TOC entry 3605 (class 0 OID 16621)
-- Dependencies: 223
-- Data for Name: orders; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.orders (id, outlet_id, table_id, status, created_at) FROM stdin;
\.


--
-- TOC entry 3599 (class 0 OID 16538)
-- Dependencies: 217
-- Data for Name: outlets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.outlets (id, name, owner_id, created_at, address_id) FROM stdin;
15c4bdc8-984d-4b8f-85bc-8dada8265586	semoga jadi	75a038e2-a188-4d55-8b65-149742dabf1c	2026-04-20 15:28:32.541142	25d7921b-919d-47ef-a33a-23ecda13caf5
2743702f-5a05-40bb-aea0-6a85edebc809	tengxiii	75a038e2-a188-4d55-8b65-149742dabf1c	2026-04-20 15:29:00.541549	bb0c54fb-353f-4dd7-9966-43143ea66044
\.


--
-- TOC entry 3603 (class 0 OID 16588)
-- Dependencies: 221
-- Data for Name: product_categories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.product_categories (product_id, category_id) FROM stdin;
\.


--
-- TOC entry 3602 (class 0 OID 16576)
-- Dependencies: 220
-- Data for Name: products; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.products (id, name, price, outlet_id, created_at) FROM stdin;
\.


--
-- TOC entry 3610 (class 0 OID 16740)
-- Dependencies: 228
-- Data for Name: qr_code_tables; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qr_code_tables (id, qr_id, table_id, created_at) FROM stdin;
\.


--
-- TOC entry 3609 (class 0 OID 16706)
-- Dependencies: 227
-- Data for Name: qr_codes; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.qr_codes (id, outlet_id, slug, created_at, expired_at) FROM stdin;
\.


--
-- TOC entry 3604 (class 0 OID 16603)
-- Dependencies: 222
-- Data for Name: tables; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tables (id, name, outlet_id, status, token, location, created_at) FROM stdin;
39fb1cd6-a32e-4146-b2a1-00125c1d4904	meja 10	15c4bdc8-984d-4b8f-85bc-8dada8265586	available	\N	\N	2026-05-05 16:39:45.415719
\.


--
-- TOC entry 3607 (class 0 OID 16655)
-- Dependencies: 225
-- Data for Name: transactions; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.transactions (id, order_id, payment_method, status, amount, created_at) FROM stdin;
\.


--
-- TOC entry 3600 (class 0 OID 16550)
-- Dependencies: 218
-- Data for Name: user_outlets; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.user_outlets (user_id, outlet_id, role) FROM stdin;
75a038e2-a188-4d55-8b65-149742dabf1c	15c4bdc8-984d-4b8f-85bc-8dada8265586	owner
75a038e2-a188-4d55-8b65-149742dabf1c	2743702f-5a05-40bb-aea0-6a85edebc809	owner
\.


--
-- TOC entry 3598 (class 0 OID 16527)
-- Dependencies: 216
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, email, password, created_at, provider, provider_id) FROM stdin;
2c7eabf9-63a9-4b08-b822-3be8611c90ce	kayu	admin@gmail.com	$2b$12$/pB/TaJSm4JzC5tQtVSm6eGc4ncCeDEC1LAZig6k19eMcN.QX53mm	2026-04-11 15:00:05.874609	\N	\N
151ed1f3-f433-4731-aab9-242b612b7179	tengsi	coba1@gmail.com	$2b$12$22mZB9rOgMA0VMBONAYx7OzZ3.CQgsGEvWmVI3ADfDfYPZzF3XfKa	2026-04-14 01:44:52.409065	\N	\N
75a038e2-a188-4d55-8b65-149742dabf1c	Muhammad Rizky	20240810023@uniku.ac.id	\N	2026-04-19 17:00:17.403292	google	117894576446598891641
\.


--
-- TOC entry 3413 (class 2606 OID 16684)
-- Name: addresses addresses_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.addresses
    ADD CONSTRAINT addresses_pkey PRIMARY KEY (id);


--
-- TOC entry 3390 (class 2606 OID 16570)
-- Name: categories categories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_pkey PRIMARY KEY (id);


--
-- TOC entry 3376 (class 2606 OID 16700)
-- Name: users email_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT email_unique UNIQUE (email);


--
-- TOC entry 3432 (class 2606 OID 16799)
-- Name: invites invites_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_pkey PRIMARY KEY (id);


--
-- TOC entry 3434 (class 2606 OID 16801)
-- Name: invites invites_token_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_token_key UNIQUE (token);


--
-- TOC entry 3407 (class 2606 OID 16644)
-- Name: order_items order_items_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_pkey PRIMARY KEY (id);


--
-- TOC entry 3404 (class 2606 OID 16628)
-- Name: orders orders_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_pkey PRIMARY KEY (id);


--
-- TOC entry 3384 (class 2606 OID 16544)
-- Name: outlets outlets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT outlets_pkey PRIMARY KEY (id);


--
-- TOC entry 3396 (class 2606 OID 16592)
-- Name: product_categories product_categories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_pkey PRIMARY KEY (product_id, category_id);


--
-- TOC entry 3394 (class 2606 OID 16582)
-- Name: products products_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_pkey PRIMARY KEY (id);


--
-- TOC entry 3427 (class 2606 OID 16746)
-- Name: qr_code_tables qr_code_tables_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_pkey PRIMARY KEY (id);


--
-- TOC entry 3419 (class 2606 OID 16716)
-- Name: qr_codes qr_codes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT qr_codes_pkey PRIMARY KEY (id);


--
-- TOC entry 3421 (class 2606 OID 16718)
-- Name: qr_codes qr_codes_slug_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT qr_codes_slug_key UNIQUE (slug);


--
-- TOC entry 3399 (class 2606 OID 16611)
-- Name: tables tables_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_pkey PRIMARY KEY (id);


--
-- TOC entry 3401 (class 2606 OID 16812)
-- Name: tables tables_token_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_token_unique UNIQUE (token);


--
-- TOC entry 3409 (class 2606 OID 16664)
-- Name: transactions transactions_order_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_order_id_key UNIQUE (order_id);


--
-- TOC entry 3411 (class 2606 OID 16662)
-- Name: transactions transactions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_pkey PRIMARY KEY (id);


--
-- TOC entry 3386 (class 2606 OID 16696)
-- Name: outlets unique_outlet_address; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT unique_outlet_address UNIQUE (address_id);


--
-- TOC entry 3429 (class 2606 OID 16748)
-- Name: qr_code_tables unique_qr_table; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT unique_qr_table UNIQUE (qr_id, table_id);


--
-- TOC entry 3423 (class 2606 OID 16735)
-- Name: qr_codes unique_slug; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT unique_slug UNIQUE (slug);


--
-- TOC entry 3388 (class 2606 OID 16554)
-- Name: user_outlets user_outlets_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_pkey PRIMARY KEY (user_id, outlet_id);


--
-- TOC entry 3378 (class 2606 OID 16537)
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- TOC entry 3380 (class 2606 OID 16535)
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- TOC entry 3382 (class 2606 OID 16705)
-- Name: users users_provider_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_provider_id_key UNIQUE (provider_id);


--
-- TOC entry 3414 (class 1259 OID 16698)
-- Name: idx_addresses_lat_lng; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_lat_lng ON public.addresses USING btree (latitude, longitude);


--
-- TOC entry 3415 (class 1259 OID 16697)
-- Name: idx_addresses_province; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_addresses_province ON public.addresses USING btree (province);


--
-- TOC entry 3391 (class 1259 OID 16671)
-- Name: idx_categories_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_categories_outlet ON public.categories USING btree (outlet_id);


--
-- TOC entry 3430 (class 1259 OID 16807)
-- Name: idx_invites_token; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_invites_token ON public.invites USING btree (token);


--
-- TOC entry 3405 (class 1259 OID 16674)
-- Name: idx_order_items_order; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_order_items_order ON public.order_items USING btree (order_id);


--
-- TOC entry 3402 (class 1259 OID 16673)
-- Name: idx_orders_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_orders_outlet ON public.orders USING btree (outlet_id);


--
-- TOC entry 3392 (class 1259 OID 16670)
-- Name: idx_products_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_products_outlet ON public.products USING btree (outlet_id);


--
-- TOC entry 3424 (class 1259 OID 16759)
-- Name: idx_qr_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_id ON public.qr_code_tables USING btree (qr_id);


--
-- TOC entry 3416 (class 1259 OID 16732)
-- Name: idx_qr_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_outlet ON public.qr_codes USING btree (outlet_id);


--
-- TOC entry 3417 (class 1259 OID 16731)
-- Name: idx_qr_slug; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_qr_slug ON public.qr_codes USING btree (slug);


--
-- TOC entry 3425 (class 1259 OID 16760)
-- Name: idx_table_id; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_table_id ON public.qr_code_tables USING btree (table_id);


--
-- TOC entry 3397 (class 1259 OID 16672)
-- Name: idx_tables_outlet; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX idx_tables_outlet ON public.tables USING btree (outlet_id);


--
-- TOC entry 3439 (class 2606 OID 16571)
-- Name: categories categories_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.categories
    ADD CONSTRAINT categories_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3435 (class 2606 OID 16690)
-- Name: outlets fk_outlets_address; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT fk_outlets_address FOREIGN KEY (address_id) REFERENCES public.addresses(id) ON DELETE CASCADE;


--
-- TOC entry 3450 (class 2606 OID 16749)
-- Name: qr_code_tables fk_qr; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT fk_qr FOREIGN KEY (qr_id) REFERENCES public.qr_codes(id) ON DELETE CASCADE;


--
-- TOC entry 3449 (class 2606 OID 16721)
-- Name: qr_codes fk_qr_outlet; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_codes
    ADD CONSTRAINT fk_qr_outlet FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3451 (class 2606 OID 16754)
-- Name: qr_code_tables fk_table; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT fk_table FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE CASCADE;


--
-- TOC entry 3454 (class 2606 OID 16802)
-- Name: invites invites_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.invites
    ADD CONSTRAINT invites_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3446 (class 2606 OID 16645)
-- Name: order_items order_items_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id) ON DELETE CASCADE;


--
-- TOC entry 3447 (class 2606 OID 16650)
-- Name: order_items order_items_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.order_items
    ADD CONSTRAINT order_items_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id) ON DELETE SET NULL;


--
-- TOC entry 3444 (class 2606 OID 16629)
-- Name: orders orders_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3445 (class 2606 OID 16634)
-- Name: orders orders_table_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.orders
    ADD CONSTRAINT orders_table_id_fkey FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE SET NULL;


--
-- TOC entry 3436 (class 2606 OID 16545)
-- Name: outlets outlets_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.outlets
    ADD CONSTRAINT outlets_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- TOC entry 3441 (class 2606 OID 16598)
-- Name: product_categories product_categories_category_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_category_id_fkey FOREIGN KEY (category_id) REFERENCES public.categories(id) ON DELETE CASCADE;


--
-- TOC entry 3442 (class 2606 OID 16593)
-- Name: product_categories product_categories_product_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.product_categories
    ADD CONSTRAINT product_categories_product_id_fkey FOREIGN KEY (product_id) REFERENCES public.products(id) ON DELETE CASCADE;


--
-- TOC entry 3440 (class 2606 OID 16583)
-- Name: products products_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.products
    ADD CONSTRAINT products_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3452 (class 2606 OID 16780)
-- Name: qr_code_tables qr_code_tables_qr_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_qr_id_fkey FOREIGN KEY (qr_id) REFERENCES public.qr_codes(id) ON DELETE CASCADE;


--
-- TOC entry 3453 (class 2606 OID 16785)
-- Name: qr_code_tables qr_code_tables_table_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.qr_code_tables
    ADD CONSTRAINT qr_code_tables_table_id_fkey FOREIGN KEY (table_id) REFERENCES public.tables(id) ON DELETE CASCADE;


--
-- TOC entry 3443 (class 2606 OID 16616)
-- Name: tables tables_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tables
    ADD CONSTRAINT tables_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3448 (class 2606 OID 16665)
-- Name: transactions transactions_order_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.transactions
    ADD CONSTRAINT transactions_order_id_fkey FOREIGN KEY (order_id) REFERENCES public.orders(id) ON DELETE CASCADE;


--
-- TOC entry 3437 (class 2606 OID 16560)
-- Name: user_outlets user_outlets_outlet_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_outlet_id_fkey FOREIGN KEY (outlet_id) REFERENCES public.outlets(id) ON DELETE CASCADE;


--
-- TOC entry 3438 (class 2606 OID 16555)
-- Name: user_outlets user_outlets_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_outlets
    ADD CONSTRAINT user_outlets_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


-- Completed on 2026-05-19 22:40:45 WIB

--
-- PostgreSQL database dump complete
--

\unrestrict PQbuxCBbndQ5If6xdbJvHkJKnjWDPci8nGLFqtbdqoda4MnCjmyouC73XPlhnSH

