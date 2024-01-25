--
-- PostgreSQL database dump
--

-- Dumped from database version 16.0
-- Dumped by pg_dump version 16.0

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: employee; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.employee (
    eid integer NOT NULL,
    ename text NOT NULL,
    dno integer NOT NULL,
    esal integer NOT NULL,
    age integer NOT NULL,
    phone text NOT NULL
);


ALTER TABLE public.employee OWNER TO postgres;

--
-- Data for Name: employee; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.employee (eid, ename, dno, esal, age, phone) FROM stdin;
101	ALADE JOY	2	250000	33	080203089832
100	MUSTAPHA ALI	32	175000	32	08063285831
107	ALOKWE MARTIN	7	380000	48	07090082812
97	DANKADE AMINAT	5	550000	48	09023688832
108	JOSIAH JOSHUA	1	120000	30	08053189131
102	MANKINDE MARY	2	450000	55	09023487830
120	ADELEKE JANE	4	200000	38	07061045862
122	OSAHON MARK	6	320000	44	08022289842
117	SULEMAN AJAYI	32	800000	50	07030089811
104	KUTI LAWAL	1	750000	35	09145689842
\.


--
-- Name: employee employee_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employee
    ADD CONSTRAINT employee_pkey PRIMARY KEY (eid);


--
-- PostgreSQL database dump complete
--

