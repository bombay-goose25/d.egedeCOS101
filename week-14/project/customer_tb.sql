--
-- PostgreSQL database dump
--

\restrict fYV5yHdXxB3B35KBaRGI1yjGxSPb1nmTUEYdXLzyNfEpkaTu3N8QRsAuOGWqMwr

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile text NOT NULL,
    eid integer NOT NULL,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	8055089112	102	5
111	Lilian Jaiye	35	l_jaiye@gmail.com	8055089844	100	3
112	Arthur Musa	50	a_musa@gmail.com	8055089334	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	8055087483	100	2
114	Mary Mapa	33	m_mapa@gmail.com	8055085738	120	5
115	Oghene Agor	50	0_agor@gmail.com	8055083837	117	11
116	Adams Bree	33	a_bree@gmail.com	8055088473	102	1
117	Okafor Matthias	45	o_matthias@gmail.com	8077282829	120	10
118	Samson Adeke	65	s_adeke@gmail.com	8075554829	117	11
119	Lamal Tamire	35	l_tamire@gmail.com	8070383833	107	5
120	James Job	44	j_job@gmail.com	8070385984	100	8
121	Mat Jaka	44	m_jaka@gmail.com	8070383444	104	2
122	Lamal Tamire	35	l_tamire@gmail.com	8070383833	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- PostgreSQL database dump complete
--

\unrestrict fYV5yHdXxB3B35KBaRGI1yjGxSPb1nmTUEYdXLzyNfEpkaTu3N8QRsAuOGWqMwr

