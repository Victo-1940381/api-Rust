

drop table if exists public.equipe cascade;
create table public.equipe(
id SERIAL PRIMARY KEY,
nom VARCHAR(255) not null UNIQUE
);
drop table if exists public.personnes cascade;
create table public.personnes(
id SERIAL PRIMARY KEY,
nom VARCHAR(255) not null,
prenom VARCHAR(255) not null,
age INTEGER  not null constraint age_positif check (age >= 0),
equipe Serial references public.equipe(id),
is_chef bool not null
);
drop  table  if exists public.projets cascade;
create table public.projets(
 id SERIAL PRIMARY KEY,
 nom VARCHAR(255) not null UNIQUE,
 date_Debut date not null,
 date_de_fin date constraint date_plus_grande check(date_de_fin > date_Debut),
 fini bool not null,
 equipeResponsables Serial references public.equipe(id)
);


drop table if exists public.taches cascade;
create table public.taches(
id SERIAL primary key,
titre varchar(255) not null,
description Text not null,
id_projet Serial references public.projets(id),
terminer bool not null
);
drop table if exists public.soustaches cascade;
create table public.soustaches(
id SERIAL primary key,
titre VARCHAR(255) not null,
description Text not null,
responsable Serial references public.personnes(id),
id_tache Serial references public.taches(id),
terminer bool not null
);
insert into public.equipe (nom)
values ('les codeurs et gameurs'),
('les gars de moteurs');

insert into public.personnes (prenom,nom,age,equipe,is_chef) 
values ('David','Brown',45,1,false),
('Peter','Ward',29,1,false),
('Alexandre','Laroche',33,1,false),
('Ludovic','Provost-Labbé',25,1,true),
('Ethan','Diaz',56,1,false),
('Sara','Rochefort',25,2,false),
('Zack','Routier',45,2,false),
('Luca','Bujold',37,2,false),
('Ella','Paquette',55,2,false),
('Pierre-luc','Héon',25,2,true);

insert into public.projets (nom,date_Debut,fini,equipeResponsables)
values('cration dun jeu avec unity 3d meilleur que GTA5','2025-09-17',false,1),
('montage dune toyota supra de a-z a partir de zero','2024-06-13',false,2);

insert into public.taches (titre,description,id_projet,terminer)
values ('creer les pnj','on doit creer tout les pnj leur véhicule, comportement...',1,false),
('creer les décors','on doit creer tout les batiment et decors',1,false),
('creer histoire','on doit creer lhistoire du jeux',1,false),
('monter le frame du char','on doit monter tout le dessou du char',2,false),
('monter le moteur','mon doit monter le moteur et ses piece', 2,false),
('monter la caroserrie','on doit monter la carroserie sur le char',2,false);

insert into Public.soustaches (titre,description,responsable,id_tache,terminer)
values('creer les voiture','on doit creer les voiture',1,1,false),
('creer les  différent pnj','on doit creer les différent modèle 3d des pnj',2,1,false),
('creer les comportement des pnj', 'on doit creer les comportement des pnj',3,1,false),
('creer les batiment','on doit creer les batiment',5,2,false),
('creer les décors','on doit creer tout ce qui est décor ',4,2,false),
('inventer une histoire','on doit inventer une bonne histore captivante',1,3,false),
('creer les items', 'on doit creer les item nessesaire au jeu',4,3,false),
('monter echapement','on doit monter tout le pot dechapement du char',6,4,false),
('monter les roue','on doit monter les roue du vehicule et tout ce quyi vien avec',8,4,true),
('monter les suspencion','on doit monter la suspencion ',10,4,true),
('on doit monter les piston','on doit monter les piston',9,5,false),
('on doit monter les arbre a came','on doit monter les arbre a came et les valve',6,5,false),
('on doit mettre les bougie dallumage','on doit mettre les bougie dallumage',9,5,false),
('on doit mettre la carosserie','on doit mettre la carossrrie sur le char',10,6,false),
('lumière','on doit mettre les lumière sur le char',7,6,false),
('peindre','on doit peindre le char',9,6,false);