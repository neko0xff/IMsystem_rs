/*!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-11.4.2-MariaDB, for Linux (x86_64)
--
-- Host: 127.0.0.1    Database: noteDB
-- ------------------------------------------------------
-- Server version	11.4.2-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `AndonLog`
--

DROP TABLE IF EXISTS `AndonLog`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `AndonLog` (
  `ID` int(11) NOT NULL AUTO_INCREMENT,
  `WorkcenterID` varchar(50) NOT NULL,
  `WorkcenterName` text DEFAULT NULL,
  `StatusIndex` int(11) NOT NULL,
  `AlarmName` text DEFAULT NULL,
  `OldStatus` text NOT NULL,
  `NewStatus` text NOT NULL,
  `ChangeDateTime` datetime NOT NULL,
  `AlarmStartTime` datetime DEFAULT NULL,
  `AlarmEndTime` datetime DEFAULT NULL,
  `AlarmStartText1` text DEFAULT NULL,
  `AlarmStartText2` text DEFAULT NULL,
  `AlarmStartText3` text DEFAULT NULL,
  `AlarmEndText1` text DEFAULT NULL,
  `AlarmEndText2` text DEFAULT NULL,
  `AlarmEndText3` text DEFAULT NULL,
  `AlarmEndText4` text DEFAULT NULL,
  PRIMARY KEY (`ID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AndonLog`
--

LOCK TABLES `AndonLog` WRITE;
/*!40000 ALTER TABLE `AndonLog` DISABLE KEYS */;
/*!40000 ALTER TABLE `AndonLog` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Localization`
--

DROP TABLE IF EXISTS `Localization`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Localization` (
  `Id` varchar(50) NOT NULL,
  `English` text DEFAULT NULL,
  `Translation` text DEFAULT NULL,
  PRIMARY KEY (`Id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Localization`
--

LOCK TABLES `Localization` WRITE;
/*!40000 ALTER TABLE `Localization` DISABLE KEYS */;
INSERT INTO `Localization` VALUES
('AddWorkcenter','Add workcenter','Přidej pracoviště'),
('AlarmEndDate','Ending Date','Koncové datum'),
('AlarmEndTime','Alarm End','Konec alarmu'),
('AlarmHistory','Alarm History for workcenter ','Historie alarmů na pracovišti '),
('AlarmLog','Alarm Log','Záznamy'),
('AlarmName','Alarm Type','Typ alarmu'),
('AlarmOverview','Alarm Overview','Přehled alarmů'),
('AlarmRow','Alarm Row','Pořadí alarmu'),
('AlarmStartDate','Starting Date','Počáteční datum'),
('AlarmStartDetails','Enter details to start the alarm','Zadej detaily pro spuštění alarmu'),
('AlarmStartTime','Alarm Start','Začátek alarmu'),
('AlarmStatistics','Alarm Statistics','Statistiky alarmů'),
('AlarmTypeEnabled','Alarm Type Enabled','Typ alarmu aktivní'),
('AlarmTypeIcon','Alarm Type Icon','Ikona typu alarmu'),
('AlarmTypeName','Alarm Type Name','Jméno typu alarmu'),
('ALARMTYPES','ALARM TYPES','TYPY ALARMŮ'),
('AlarmTypesForWorkcenter','Alarm types for Workcenter ','Typy alarmů pro pracoviště '),
('All','All','Vše'),
('Cancel','Cancel','Zrušit'),
('ChooseIcon','Choose Icon','Vyber ikonu'),
('Close','Close','Zavřít'),
('ConfirmAlarm','Confirm Alarm','Potvrdit alarm'),
('CurrentValue','Current value','Současná hodnota'),
('DefaultText','Default text (English)','Standardní text (angličtina)'),
('DefineAlarmStartDetails','Define alarm start details - ','Definuj detaily pro začátek alarmu - '),
('DeleteWorkcenter','Delete workcenter','Smaž pracovíště'),
('DetailEnabled','Detail enabled','Detail aktivní'),
('DetailsFree','Details (free text field)','Detaily (volné textové pole)'),
('DetailsOptional','Details (optional)','Detaily (nepovinné)'),
('DurationMin','Alarm Duration (min)','Trvání alarmu (min)'),
('Enabled','Enabled','Aktivní'),
('EnabledForEnd','Enabled for alarm end','Zapnuty pro konec alarmů'),
('EnabledForStart','Enabled for alarm start','Zapnuty pro začátek alarmů'),
('EnabledForStartEnd','Enabled for alarm start & end','Zapnuty pro začátek i konec alarmů'),
('FailureDetails','Failure Details','Detaily problému'),
('FailureLocation','Failure Location','Místo problému'),
('FailureLocationForWorkcenter','Failure locations for Workcenter ','Místa problémů pro pracoviště '),
('FailureType','Failure Type','Typ problému'),
('FailureTypesForworkcenter','Failure types for Workcenter ','Typy problémů pro pracoviště '),
('INTERFACESETTINGS','INTERFACE SETTINGS','NASTAVENÍ PROSTŘEDÍ'),
('LOCALIZATION','LOCALIZATION','PŘEKLAD'),
('LocalizedText','Localized text (translation)','Přeložený text'),
('MTBF','Mean time between failures (MTBF)','Střední doba mezi poruchami (MTBF)'),
('MTTR','Mean time to repair (MTTR)','Střední doba opravy (MTTR)'),
('No','No','Ne'),
('NoDetailsEnabled','No details enabled','Detaily vypnuty'),
('NoOfFinishedAlarms','Nr. of finished alarms','Počet ukončených alarmů'),
('NoWorkcentersWithActiveAlarms','--  No workcenters with active alarms --','-- Žádná pracovíště s aktivními alarmy --'),
('NumberOfAlarms','Number of Alarms','Počet alarmů'),
('OptionsSeparated','Options (separated by | )','Volby (oddělené | )'),
('PercentageOfTotal','Percentage of Total','Procento z celku'),
('ResetToDefaultSettings','Reset to default settings','Vrátit původní nastavení'),
('SaveAndClose','Save and close','Uložit a zavřít'),
('SaveID','Save ID','Ulož ID'),
('SaveName','Save name','Ulož jméno'),
('SaveText','Ulož text','Save text'),
('SelectOption','-- Select option --','-- Zvol možnost --'),
('SettingName','Setting name','Jméno nastavení'),
('Settings','Settings','Nastavení'),
('ShowLogEntries','Show Log Entries','Ukaž záznamy'),
('ShowOnlyActiveAlarms',' Show only workcenters with active alarms',' Zobraz pouze pracoviště s aktivními alarmy'),
('ShowOnlyFinishedAlarms','Show only finished alarms:','Ukaž pouze ukončené alarmy:'),
('ShowStatistics','Show Statistics','Ukaž statistiku'),
('TerminalHeader','Andon Terminal for workcenter','Andon terminál pro pracoviště'),
('TerminalInstruction1','⇒ If a problem arises, click on the relevant green field.','⇒ Pokud nastane problém, klikněte na příslušné zelené pole.'),
('TerminalInstruction2','⇒ The field turns red, showing the time since the problem started.','⇒ Pole zčervená a ukazuje čas od začátku problému.'),
('TerminalInstruction3','⇒ After resolving the issue, click on the red field to change it back to green. ','⇒ Po vyřešení problému klikněte na červené pole a změňte jej zpět na zelené.'),
('TypeOfDetail','Type of detail','Typ detailu'),
('WhatIsAndon','What is Andon?','Co je to Andon?'),
('Workcenter','Workcenter','Pracoviště'),
('WorkcenterID','Workcenter ID','ID pracoviště'),
('WorkcenterIDUnique','Workcenter ID (unique)','ID pracoviště (jedinečné)'),
('WorkcenterName','Workcenter Name','Název pracoviště'),
('WorkcenterRow','Workcenter Row','Pořadí pracoviště'),
('WORKCENTERS','WORKCENTERS','PRACOVIŠTĚ'),
('Yes','Yes','Ano');
/*!40000 ALTER TABLE `Localization` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Settings`
--

DROP TABLE IF EXISTS `Settings`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `Settings` (
  `SettingID` int(11) NOT NULL,
  `SettingName` varchar(50) DEFAULT NULL,
  `CurrentSetting` text DEFAULT NULL,
  `PossibleSettings` text DEFAULT NULL,
  `DefaultSetting` text DEFAULT NULL,
  PRIMARY KEY (`SettingID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Settings`
--

LOCK TABLES `Settings` WRITE;
/*!40000 ALTER TABLE `Settings` DISABLE KEYS */;
INSERT INTO `Settings` VALUES
(1,'Language','English','English|Translation','English'),
(2,'Show workcenter name?','Yes','Yes|No','Yes'),
(3,'Show only workcenters with alarms in Overview?','No','Yes|No','No');
/*!40000 ALTER TABLE `Settings` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `StatusDefinition`
--

DROP TABLE IF EXISTS `StatusDefinition`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `StatusDefinition` (
  `StatusRow` int(11) NOT NULL,
  `StatusName` varchar(20) NOT NULL,
  `StatusEnabled` bit(1) NOT NULL,
  `StatusDetailsEnabled` int(11) NOT NULL,
  `IconName` varchar(50) DEFAULT NULL,
  `AlarmStartText1Structure` text DEFAULT NULL,
  `AlarmStartText2Structure` text DEFAULT NULL,
  `AlarmStartText3Structure` text DEFAULT NULL,
  `AlarmEndText1Structure` text DEFAULT NULL,
  `AlarmEndText2Structure` text DEFAULT NULL,
  `AlarmEndText3Structure` text DEFAULT NULL,
  `AlarmEndText4Structure` text DEFAULT NULL,
  PRIMARY KEY (`StatusRow`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `StatusDefinition`
--

LOCK TABLES `StatusDefinition` WRITE;
/*!40000 ALTER TABLE `StatusDefinition` DISABLE KEYS */;
INSERT INTO `StatusDefinition` VALUES
(1,'Machine trouble','',1,'fa fa-cogs','ON|Loading|Robot|Pressing|Machining|Assembly|Welding|Soldering|Cutting|Injection Molding|Extrusion|Painting|Coating|Laser Cutting|Testing|Quality Control|Material Handling|Packaging|Heat Treating|Casting|Forming|Grinding|Deburring|Polishing','ON|Mechanical failure|Electrical failure|Software failure','ON|','OFF','OFF','OFF','OFF'),
(2,'Quality issue','',0,'fa fa-search','OFF','OFF','OFF','OFF','OFF','OFF','OFF'),
(3,'Material shortage','',1,'fa fa-bars','OFF','OFF','ON','OFF','OFF','OFF','OFF'),
(4,'Process abnormality','',0,'fa fa-thermometer-full','OFF','OFF','OFF','OFF','OFF','OFF','OFF'),
(5,'Teamleader needed','',0,'fa fa-street-view','OFF','OFF','OFF','OFF','OFF','OFF','OFF');
/*!40000 ALTER TABLE `StatusDefinition` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `WorkcenterList`
--

DROP TABLE IF EXISTS `WorkcenterList`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `WorkcenterList` (
  `WorkcenterRow` int(11) NOT NULL,
  `WorkcenterID` varchar(50) NOT NULL,
  `WorkcenterName` varchar(50) NOT NULL,
  `Status1` text DEFAULT NULL,
  `Status2` text DEFAULT NULL,
  `Status3` text DEFAULT NULL,
  `Status4` text DEFAULT NULL,
  `Status5` text DEFAULT NULL,
  PRIMARY KEY (`WorkcenterRow`),
  UNIQUE KEY `WorkcenterID` (`WorkcenterID`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `WorkcenterList`
--

LOCK TABLES `WorkcenterList` WRITE;
/*!40000 ALTER TABLE `WorkcenterList` DISABLE KEYS */;
INSERT INTO `WorkcenterList` VALUES
(1,'A-003','Mixing Station','green','green','green','green','green'),
(2,'A-012','Heating Chamber','green','green','green','green','green'),
(3,'B-015','Curing Area','green','green','green','green','green'),
(4,'B-019','Assembly','green','green','green','green','green'),
(5,'C-020','Inspection & Packing','green','green','green','green','green');
/*!40000 ALTER TABLE `WorkcenterList` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `notes`
--

DROP TABLE IF EXISTS `notes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `notes` (
  `id` char(36) NOT NULL,
  `title` varchar(255) NOT NULL,
  `content` text NOT NULL,
  `is_published` tinyint(1) NOT NULL DEFAULT 0,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `title` (`title`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `notes`
--

LOCK TABLES `notes` WRITE;
/*!40000 ALTER TABLE `notes` DISABLE KEYS */;
INSERT INTO `notes` VALUES
('b7b82358-7350-4594-8f93-af418f41ce7a','a note','here some reminder, mention @raditzlawliet',0,'2024-05-12 00:53:46','2024-05-12 00:53:46');
/*!40000 ALTER TABLE `notes` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `todos`
--

DROP TABLE IF EXISTS `todos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `todos` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `description` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `todos`
--

LOCK TABLES `todos` WRITE;
/*!40000 ALTER TABLE `todos` DISABLE KEYS */;
INSERT INTO `todos` VALUES
(1,'user1','Add timer'),
(2,'user1','Add timer'),
(3,'user2','Add timer');
/*!40000 ALTER TABLE `todos` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `loginname` varchar(255) NOT NULL,
  `mail` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `users`
--

LOCK TABLES `users` WRITE;
/*!40000 ALTER TABLE `users` DISABLE KEYS */;
INSERT INTO `users` VALUES
(1,'user1','$argon2id$v=19$m=19456,t=2,p=1$XnFSbbheG5O9QHv582mBaw$U3Q5VYSUHa2paFDC5BFwqUmfqLDopyKMHbsE30z/bik','user1','user1@demo.com'),
(2,'user2','$argon2id$v=19$m=19456,t=2,p=1$9QJNVjWOs04p9sfWPnn15A$SHNXe+QszUaa+eZKH3SaK1RUfgIgwBf/2vred2Y5YL0','user2','user2@demo.com'),
(3,'user3','$argon2id$v=19$m=19456,t=2,p=1$Z1DP5ZocbJ5sdGiCqefZMQ$/zbiRuFZGANj/9PUDq36JGL5Ll/QGSD14poUEt/t4Tc','user3','user3@demo.com'),
(4,'user4','$argon2id$v=19$m=19456,t=2,p=1$JD+x2FL79dlRXOiaSdwhhw$5rqOtagACdeLOBCHS2PoxXb08lz/Sy3XOGbUciyNjZk','user4','user4@demo.com');
/*!40000 ALTER TABLE `users` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2024-07-20 13:57:41
