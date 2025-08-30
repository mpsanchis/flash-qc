--
-- PostgreSQL database dump
--

\restrict btVugeRFO7a6fX2QwvOOxkiMGsupUmL43hoKp9GLR0cvTKNZeuzcdMYAOIFUlF7

-- Dumped from database version 17.6 (Postgres.app)
-- Dumped by pg_dump version 17.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2025-07-12 08:29:52.703423
20250712084226	2025-07-12 08:43:18.827178
20250712090819	2025-07-12 09:10:21.193606
20250712093343	2025-07-12 09:38:33.394866
20250719101830	2025-07-19 10:23:58.126799
\.


--
-- Data for Name: cards; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.cards (id, name) FROM stdin;
0	ronaldo
1	messi
\.


--
-- Data for Name: tags; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.tags (id, name, uuid) FROM stdin;
0	football	6741fbb1-3a0c-4387-b14c-6d60e38c12a9
1	europe	b9e703ab-afe0-4f78-b912-ca32298340cd
2	switzerland	273f5c3e-7576-4569-a15c-7581dc27604d
\.


--
-- Data for Name: card_tags_link; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.card_tags_link (card_id, tag_id) FROM stdin;
0	0
0	1
1	0
\.


--
-- Data for Name: plugins; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.plugins (id, name, link) FROM stdin;
1	kanji	PLACEHOLDER
2	standard	PLACEHOLDER
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: flashqc
--

COPY public.users (id, name, id_plugin) FROM stdin;
1	flashqc	1
2	ale	2
3	miguel	2
\.


--
-- Name: cards_id_seq; Type: SEQUENCE SET; Schema: public; Owner: flashqc
--

SELECT pg_catalog.setval('public.cards_id_seq', 1, false);


--
-- Name: plugins_id_seq; Type: SEQUENCE SET; Schema: public; Owner: flashqc
--

SELECT pg_catalog.setval('public.plugins_id_seq', 1, false);


--
-- Name: tags_id_seq; Type: SEQUENCE SET; Schema: public; Owner: flashqc
--

SELECT pg_catalog.setval('public.tags_id_seq', 1, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: flashqc
--

SELECT pg_catalog.setval('public.users_id_seq', 4, true);


--
-- PostgreSQL database dump complete
--

\unrestrict btVugeRFO7a6fX2QwvOOxkiMGsupUmL43hoKp9GLR0cvTKNZeuzcdMYAOIFUlF7

