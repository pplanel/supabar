-- CreateTable
CREATE TABLE "_migrations" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "checksum" TEXT NOT NULL,
    "steps_applied" INTEGER NOT NULL DEFAULT 0,
    "applied_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "files" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "cas_id" TEXT NOT NULL,
    "integrity_checksum" TEXT,
    "kind" INTEGER NOT NULL DEFAULT 0,
    "size_in_bytes" TEXT NOT NULL,
    "key_id" INTEGER,
    "hidden" BOOLEAN NOT NULL DEFAULT false,
    "favorite" BOOLEAN NOT NULL DEFAULT false,
    "important" BOOLEAN NOT NULL DEFAULT false,
    "has_thumbnail" BOOLEAN NOT NULL DEFAULT false,
    "has_thumbstrip" BOOLEAN NOT NULL DEFAULT false,
    "has_video_preview" BOOLEAN NOT NULL DEFAULT false,
    "ipfs_id" TEXT,
    "comment" TEXT,
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "date_modified" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "date_indexed" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "media_data" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "pixel_width" INTEGER,
    "pixel_height" INTEGER,
    "longitude" REAL,
    "latitude" REAL,
    "fps" INTEGER,
    "capture_device_make" TEXT,
    "capture_device_model" TEXT,
    "capture_device_software" TEXT,
    "duration_seconds" INTEGER,
    "codecs" TEXT,
    "streams" INTEGER,
    CONSTRAINT "media_data_id_fkey" FOREIGN KEY ("id") REFERENCES "files" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "tags" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "pub_id" TEXT NOT NULL,
    "name" TEXT,
    "total_files" INTEGER DEFAULT 0,
    "redundancy_goal" INTEGER DEFAULT 1,
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "date_modified" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "tags_on_files" (
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "tag_id" INTEGER NOT NULL,
    "file_id" INTEGER NOT NULL,

    PRIMARY KEY ("tag_id", "file_id"),
    CONSTRAINT "tags_on_files_file_id_fkey" FOREIGN KEY ("file_id") REFERENCES "files" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT "tags_on_files_tag_id_fkey" FOREIGN KEY ("tag_id") REFERENCES "tags" ("id") ON DELETE NO ACTION ON UPDATE NO ACTION
);

-- CreateTable
CREATE TABLE "jobs" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "node_id" INTEGER NOT NULL,
    "action" INTEGER NOT NULL,
    "status" INTEGER NOT NULL DEFAULT 0,
    "task_count" INTEGER NOT NULL DEFAULT 1,
    "completed_task_count" INTEGER NOT NULL DEFAULT 0,
    "date_created" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "date_modified" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "seconds_elapsed" INTEGER NOT NULL DEFAULT 0
);

-- CreateIndex
CREATE UNIQUE INDEX "_migrations_checksum_key" ON "_migrations"("checksum");

-- CreateIndex
CREATE UNIQUE INDEX "files_cas_id_key" ON "files"("cas_id");

-- CreateIndex
CREATE UNIQUE INDEX "files_integrity_checksum_key" ON "files"("integrity_checksum");

-- CreateIndex
CREATE UNIQUE INDEX "tags_pub_id_key" ON "tags"("pub_id");
