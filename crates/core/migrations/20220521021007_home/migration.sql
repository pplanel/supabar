/*
  Warnings:

  - Added the required column `home_dir` to the `users` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_users" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "username" TEXT NOT NULL,
    "home_dir" TEXT NOT NULL,
    "index_dir" TEXT NOT NULL,
    "data_dir" TEXT NOT NULL,
    "hostname" TEXT,
    "platform" INTEGER NOT NULL DEFAULT 0,
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO "new_users" ("data_dir", "date_created", "hostname", "id", "index_dir", "platform", "username") SELECT "data_dir", "date_created", "hostname", "id", "index_dir", "platform", "username" FROM "users";
DROP TABLE "users";
ALTER TABLE "new_users" RENAME TO "users";
CREATE UNIQUE INDEX "users_username_key" ON "users"("username");
PRAGMA foreign_key_check;
PRAGMA foreign_keys=ON;
