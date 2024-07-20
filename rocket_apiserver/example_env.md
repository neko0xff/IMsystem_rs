環境設置
===

## 變數
### 步驟
1. 請建立`.env`等檔案，以方便程式啟動時從中讀取變數
2. 請加入如下指定內容
### 相關內容
- Rust support placeholder For sqlx cli and rust sqlx
```
MYSQL_DATABASE=     # 資料庫名稱
MYSQL_USER=         # 資料庫帳戶名
MYSQL_PASSWORD=     # 資料庫帳戶密碼
DBServer_IP=0.0.0.0       # 資料庫伺服器位置
DBServer_PORT=3306        # 資料庫伺服器通訊埠

DATABASE_URL=mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@${DBServer_IP}:${DBServer_PORT}/${MYSQL_DATABASE}
```

## 資料庫表格&內容
- 備份檔: `rocket_apiserver/data/noteDB_v[x].sql.gz`
- 手動
   * 備份: `mysqldump -h 127.0.0.1 -u master -p noteDB > noteDB_v[x].sql`
   * 還原: `mariadb -u master -p noteDB < noteDB_v2.sql`
