-- Migration: Create Garrison Tables
-- Purpose: Store conversation history entries with metadata and optional embeddings
-- Version: 001
-- Date: 2026-01-23

-- Main garrison entries table
CREATE TABLE IF NOT EXISTS garrison_entries (
    id TEXT PRIMARY KEY NOT NULL,
    paladin_id TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('system', 'user', 'assistant', 'tool')),
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    token_count INTEGER,
    metadata TEXT, -- JSON blob for flexible metadata
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for efficient retrieval by paladin and timestamp
CREATE INDEX IF NOT EXISTS idx_paladin_timestamp 
ON garrison_entries(paladin_id, timestamp DESC);

-- Index for filtering by role
CREATE INDEX IF NOT EXISTS idx_paladin_role 
ON garrison_entries(paladin_id, role);

-- Index for efficient recent entry retrieval
CREATE INDEX IF NOT EXISTS idx_created_at 
ON garrison_entries(created_at DESC);

-- Full-text search virtual table for content search
CREATE VIRTUAL TABLE IF NOT EXISTS garrison_search 
USING fts5(
    content,
    content='garrison_entries',
    content_rowid='rowid'
);

-- Triggers to keep FTS index synchronized
CREATE TRIGGER IF NOT EXISTS garrison_search_insert 
AFTER INSERT ON garrison_entries
BEGIN
    INSERT INTO garrison_search(rowid, content)
    VALUES (new.rowid, new.content);
END;

CREATE TRIGGER IF NOT EXISTS garrison_search_delete 
AFTER DELETE ON garrison_entries
BEGIN
    DELETE FROM garrison_search WHERE rowid = old.rowid;
END;

CREATE TRIGGER IF NOT EXISTS garrison_search_update 
AFTER UPDATE ON garrison_entries
BEGIN
    DELETE FROM garrison_search WHERE rowid = old.rowid;
    INSERT INTO garrison_search(rowid, content)
    VALUES (new.rowid, new.content);
END;

-- Embeddings table for vector similarity search (optional, for future enhancement)
-- Note: sqlite-vss would be used for vector operations if enabled
CREATE TABLE IF NOT EXISTS garrison_embeddings (
    entry_id TEXT PRIMARY KEY NOT NULL,
    embedding BLOB, -- Vector embedding stored as blob
    embedding_model TEXT NOT NULL,
    dimension INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (entry_id) REFERENCES garrison_entries(id) ON DELETE CASCADE
);

-- Index for embedding lookups
CREATE INDEX IF NOT EXISTS idx_embedding_model 
ON garrison_embeddings(embedding_model);

-- Metadata table for garrison configuration and statistics
CREATE TABLE IF NOT EXISTS garrison_metadata (
    paladin_id TEXT PRIMARY KEY NOT NULL,
    max_entries INTEGER NOT NULL DEFAULT 100,
    max_tokens INTEGER,
    eviction_strategy TEXT NOT NULL DEFAULT 'importance_based',
    preserve_recent_count INTEGER NOT NULL DEFAULT 10,
    total_entries INTEGER NOT NULL DEFAULT 0,
    total_tokens INTEGER NOT NULL DEFAULT 0,
    last_eviction TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Version tracking for migrations
CREATE TABLE IF NOT EXISTS garrison_schema_version (
    version INTEGER PRIMARY KEY NOT NULL,
    applied_at TEXT NOT NULL DEFAULT (datetime('now')),
    description TEXT
);

-- Insert initial schema version
INSERT OR IGNORE INTO garrison_schema_version (version, description)
VALUES (1, 'Initial garrison tables with FTS and embeddings support');
