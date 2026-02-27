-- 1. [Security] เปิดใช้งาน Extension สำหรับการสร้าง UUID
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- 2. [Security] สร้างตาราง Devices สำหรับเก็บข้อมูลอุปกรณ์
-- เราเก็บ api_key_hash เพื่อไม่ให้เห็น Key ตัวจริงในฐานข้อมูล
CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_name TEXT NOT NULL,
    api_key_hash TEXT NOT NULL UNIQUE, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. [Performance] สร้าง Index สำหรับการค้นหา API Key ในชั้น Middleware
CREATE INDEX IF NOT EXISTS idx_devices_api_key_hash ON devices (api_key_hash);

-- 4. [Performance] สร้างตาราง Activities สำหรับเก็บข้อมูล IoT
-- ใช้ JSONB เพื่อความยืดหยุ่นของ Payload และรองรับ Indexing
CREATE TABLE IF NOT EXISTS activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    activity_type VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. [Performance] สร้าง Index สำหรับการ Query ข้อมูลย้อนหลังตามช่วงเวลา
-- ช่วยให้การทำ Dashboard เรียกข้อมูลล่าสุดได้เร็วมาก
CREATE INDEX IF NOT EXISTS idx_activities_device_id_created_at 
ON activities (device_id, created_at DESC);

-- 6. [Security/Optimization] ให้สิทธิ์พื้นฐานแก่ User
-- (หมายเหตุ: ใน Production จริงควรทำผ่าน Script แยก หรือระบุ User ให้ชัดเจน)
GRANT ALL PRIVILEGES ON TABLE devices TO iot_writer;
GRANT ALL PRIVILEGES ON TABLE activities TO iot_writer;
GRANT SELECT ON TABLE devices TO iot_reader;
GRANT SELECT ON TABLE activities TO iot_reader;