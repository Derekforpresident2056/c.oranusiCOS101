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
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    cid integer NOT NULL,
    cname text NOT NULL,
    cage integer NOT NULL,
    cemail text NOT NULL,
    cmobile text NOT NULL,
    eid integer NOT NULL,
    dataid integer NOT NULL
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    dataid integer NOT NULL,
    data_size text NOT NULL,
    data_duration_days integer NOT NULL,
    data_price_naira integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation text NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

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
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno integer NOT NULL,
    pname text NOT NULL,
    pduration text NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (cid, cname, cage, cemail, cmobile, eid, dataid) FROM stdin;
110	MUSTA KARIM	35	M_KARIM@GMAIL.COM	08055089112	102	5
111	LILIAN JAIYA	43	L_JAIYA@GMAIL.COM	08055185341	100	3
112	ARTHUR MUSA	50	A_MUSA@GMAIL.COM	070550282813	107	10
113	PHILIP AKONJO	41	P_AK0NJO@GMAIL.COM	09052356772	100	2
114	MARYLENE MAPA	33	M_MAPA@GMAIL.COM	08053333551	120	5
115	OGHENERO AGOR	50	O_AGOR@GMAIL.COM	07055566774	117	11
116	ADAMS BREE	33	A_BREE@GMAIL.COM	08056765424	102	1
117	OKAFOR MATHIAS	45	O_MATHIAS@GMAIL.COM	08056763367	120	10
118	SAMSON ADELEKE	65	S_ADELEKE@GMAIL.COM	07056774423	117	11
119	LAWAL TAMIRE	35	L_TAMIRE@GMAIL.COM	09052111101	107	5
120	JAMES JOB	44	J_JOB@GMAIL.COM	08059693919	100	8
121	MATHEW JAKANDE	21	M_JAKANDE@GMAIL.COM	07051232144	120	2
122	JIMILA ADEGBOYE	20	J_ADEGBOYE@GMAIL.COM	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (dataid, data_size, data_duration_days, data_price_naira) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.2GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.5GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	ADMINISTRATION	IKEJA	44
101	2	ACCOUNT	EGBEDA	11
100	32	PACKAGING	AJAH	44
120	4	RESEARCH	V.I	33
97	5	ACCOUNT	MAGODO	22
122	6	OPERATIONS	MILE 2	44
107	7	PACKAGING	KETU	55
\.


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
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	A	9 MONTHS	102
22	B	14 MONTHS	97
33	C	16 MONTHS	120
44	D	25 MONTHS	108
55	E	9 MONTHS	107
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (cid);


--
-- Name: dataplan database_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT database_pkey PRIMARY KEY (dataid);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: employee employee_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.employee
    ADD CONSTRAINT employee_pkey PRIMARY KEY (eid);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

