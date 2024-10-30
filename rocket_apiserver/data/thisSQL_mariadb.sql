CREATE TABLE faq_guestBook (
  id int(10) unsigned NOT NULL auto_increment,
  title varchar(255) NOT NULL,
  userId varchar(50) NOT NULL,
  dateTime varchar(50) NOT NULL,
  visit int(10) NOT NULL default '0',
  report int(10) NOT NULL default '0',
  display tinyint(1) NOT NULL default '1',
  content text,
  description text,
  PRIMARY KEY  (id),
  KEY title (title),
  KEY userId (userId),
  KEY dateTime (dateTime),
  KEY visit (visit),
  KEY report (report),
  KEY display (display)
) ENGINE=InnoDB;

CREATE TABLE faq_comment (
  id int(10) unsigned NOT NULL auto_increment,
  qid int(10) NOT NULL,
  manager varchar(50) NOT NULL,
  dateTime varchar(50) NOT NULL,
  content text,
  PRIMARY KEY  (id),
  KEY qid (qid),
  KEY manager (manager),
  KEY dateTime (dateTime)
) ENGINE=InnoDB;

CREATE TABLE faq_configure (
  id int(10) unsigned NOT NULL auto_increment,
  sort int(10) NOT NULL,
  category varchar(50) NOT NULL,
  title varchar(255),
  name varchar(50),
  value varchar(50),
  encodeName varchar(50),
  dateTime varchar(50),
  type varchar(50),
  content text,
  description text,
  PRIMARY KEY  (id),
  KEY category (category),
  KEY title (title),
  KEY name (name),
  KEY value (value),
  KEY encodeName (encodeName),
  KEY dateTime (dateTime)
) ENGINE=InnoDB;

CREATE TABLE faq_userlog (
  id int(10) unsigned NOT NULL auto_increment,
  userLevel varchar(50) NOT NULL,
  lastQuestId int(10) NOT NULL default '0',
  askcount int(10) NOT NULL default '0',
  anserCount int(10) NOT NULL default '0',
  lastLoginTime varchar(50) NOT NULL,
  description text,
  PRIMARY KEY  (id),
  KEY userLevel (userLevel),
  KEY lastQuestId (lastQuestId),
  KEY askcount (askcount),
  KEY anserCount (anserCount),
  KEY lastLoginTime (lastLoginTime)
) ENGINE=InnoDB;

CREATE TABLE faq_weblog (
  id int(10) unsigned NOT NULL auto_increment,
  dateTime varchar(50) NOT NULL,
  userId varchar(50) NOT NULL,
  action text,
  PRIMARY KEY  (id),
  KEY dateTime (dateTime),
  KEY userId (userId)
) ENGINE=InnoDB;
