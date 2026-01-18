--
-- PostgreSQL database dump
--

\restrict Iw5GdHoBysmJcDn2XHFmUaVClgyzLb2PKhy2SZNjfjRV77s39S3F858M1gspzyC

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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name text CONSTRAINT employees_ename_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_salary real CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_salary, age, mobile) FROM stdin;
100	Mustapha Ali	3	175000	32	8032737363
107	Alokwe Martin	7	380000	48	8009784633
97	Dankade Aminat	5	550000	40	8064544633
108	Josiah Joshua	1	120000	30	8047388796
102	Mankinde Mary	2	450000	55	8047498349
120	Adenke Jane	4	200000	38	8047484287
122	Oshaon Mark	6	320000	44	8058577377
104	Kuti Lawal	1	750000	35	8087743543
117	Suleman Ajayi	3	800000	50	7030089981
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict Iw5GdHoBysmJcDn2XHFmUaVClgyzLb2PKhy2SZNjfjRV77s39S3F858M1gspzyC

