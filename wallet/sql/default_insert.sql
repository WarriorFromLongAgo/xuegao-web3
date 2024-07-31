
-- 假设这些数据是随机生成的或从某个源获取的
INSERT INTO blocks (hash, parent_hash, number, timestamp, rlp_bytes) VALUES
                                                                         ('0x1', '0x0', 1, '2024-07-28 12:00:00', '0xabcdef'),
                                                                         ('0x2', '0x1', 2, '2024-07-28 12:01:00', '0xbcdefa'),
                                                                         ('0x3', '0x2', 3, '2024-07-28 12:02:00', '0xcdefab'),
                                                                         ('0x4', '0x3', 4, '2024-07-28 12:03:00', '0xdefabc'),
                                                                         ('0x5', '0x4', 5, '2024-07-28 12:04:00', '0xefabcd'),
                                                                         ('0x6', '0x5', 6, '2024-07-28 12:05:00', '0xfabcde'),
                                                                         ('0x7', '0x6', 7, '2024-07-28 12:06:00', '0xabcdef'),
                                                                         ('0x8', '0x7', 8, '2024-07-28 12:07:00', '0xbcdefa'),
                                                                         ('0x9', '0x8', 9, '2024-07-28 12:08:00', '0xcdefab'),
                                                                         ('0xA', '0x9', 10, '2024-07-28 12:09:00', '0xdefabc');

INSERT INTO tokens (token_address, decimal, token_name, collect_amount, timestamp) VALUES
                                                                                       ('0x1234567890abcdef1234567890abcdef12345678', 18, 'Token1', 10.0, '2024-07-28 12:00:00'),
                                                                                       ('0x234567890abcdef1234567890abcdef123456789', 18, 'Token2', 20.0, '2024-07-28 12:01:00'),
                                                                                       ('0x34567890abcdef1234567890abcdef1234567890', 18, 'Token3', 30.0, '2024-07-28 12:02:00'),
                                                                                       ('0x4567890abcdef1234567890abcdef12345678901', 18, 'Token4', 40.0, '2024-07-28 12:03:00'),
                                                                                       ('0x567890abcdef1234567890abcdef123456789012', 18, 'Token5', 50.0, '2024-07-28 12:04:00'),
                                                                                       ('0x67890abcdef1234567890abcdef1234567890123', 18, 'Token6', 60.0, '2024-07-28 12:05:00'),
                                                                                       ('0x7890abcdef1234567890abcdef12345678901234', 18, 'Token7', 70.0, '2024-07-28 12:06:00'),
                                                                                       ('0x890abcdef1234567890abcdef123456789012345', 18, 'Token8', 80.0, '2024-07-28 12:07:00'),
                                                                                       ('0x90abcdef1234567890abcdef1234567890123456', 18, 'Token9', 90.0, '2024-07-28 12:08:00'),
                                                                                       ('0x0abcdef1234567890abcdef12345678901234567', 18, 'Token10', 100.0, '2024-07-28 12:09:00');


-- 插入测试数据
INSERT INTO addresses (user_uid, address, address_type, private_key, public_key, timestamp)
VALUES
    ('user1', '0x1234567890abcdef1234567890abcdef12345678', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:00:00'),
    ('user2', '0x234567890abcdef1234567890abcdef123456789', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:01:00'),
    ('user3', '0x34567890abcdef1234567890abcdef1234567890', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:02:00'),
    ('user4', '0x4567890abcdef1234567890abcdef12345678901', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:03:00'),
    ('user5', '0x567890abcdef1234567890abcdef123456789012', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:04:00'),
    ('user6', '0x67890abcdef1234567890abcdef1234567890123', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:05:00'),
    ('user7', '0x7890abcdef1234567890abcdef12345678901234', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:06:00'),
    ('user8', '0x890abcdef1234567890abcdef123456789012345', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:07:00'),
    ('user9', '0x90abcdef1234567890abcdef1234567890123456', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:08:00'),
    ('user10', '0x0abcdef1234567890abcdef12345678901234567', 'ETH', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef', '2024-07-28 12:09:00');



-- 插入测试数据
INSERT INTO balances (address, token_address, balance, lock_balance, timestamp)
VALUES
    ('0x1234567890abcdef1234567890abcdef12345678',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 100.0, 10.0, '2024-07-28 12:00:00'),
    ('0x234567890abcdef1234567890abcdef123456789',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 200.0, 20.0, '2024-07-28 12:01:00'),
    ('0x34567890abcdef1234567890abcdef1234567890',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 300.0, 30.0, '2024-07-28 12:02:00'),
    ('0x4567890abcdef1234567890abcdef12345678901',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 400.0, 40.0, '2024-07-28 12:03:00'),
    ('0x567890abcdef1234567890abcdef123456789012',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 500.0, 50.0, '2024-07-28 12:04:00'),
    ('0x67890abcdef1234567890abcdef1234567890123',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 600.0, 60.0, '2024-07-28 12:05:00'),
    ('0x7890abcdef1234567890abcdef12345678901234',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 700.0, 70.0, '2024-07-28 12:06:00'),
    ('0x890abcdef1234567890abcdef123456789012345',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 800.0, 80.0, '2024-07-28 12:07:00'),
    ('0x90abcdef1234567890abcdef1234567890123456',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 900.0, 90.0, '2024-07-28 12:08:00'),
    ('0x0abcdef1234567890abcdef12345678901234567',  '0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef', 1000.0, 100.0, '2024-07-28 12:09:00');



-- 插入测试数据
INSERT INTO transactions (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, tx_type, timestamp)
VALUES
    ('0xabc123', 1, '0xhash1', '0xfrom1', '0xto1', '0xtoken1', 0.1, 10.0, 0, 1, 0, '2024-07-28 12:00:00'),
    ('0xabc124', 2, '0xhash2', '0xfrom2', '0xto2', '0xtoken2', 0.2, 20.0, 1, 2, 1, '2024-07-28 12:01:00'),
    ('0xabc125', 3, '0xhash3', '0xfrom3', '0xto3', '0xtoken3', 0.3, 30.0, 0, 3, 0, '2024-07-28 12:02:00'),
    ('0xabc126', 4, '0xhash4', '0xfrom4', '0xto4', '0xtoken4', 0.4, 40.0, 1, 4, 1, '2024-07-28 12:03:00'),
    ('0xabc127', 5, '0xhash5', '0xfrom5', '0xto5', '0xtoken5', 0.5, 50.0, 0, 5, 0, '2024-07-28 12:04:00'),
    ('0xabc128', 6, '0xhash6', '0xfrom6', '0xto6', '0xtoken6', 0.6, 60.0, 1, 6, 1, '2024-07-28 12:05:00'),
    ('0xabc129', 7, '0xhash7', '0xfrom7', '0xto7', '0xtoken7', 0.7, 70.0, 0, 7, 0, '2024-07-28 12:06:00'),
    ('0xabc130', 8, '0xhash8', '0xfrom8', '0xto8', '0xtoken8', 0.8, 80.0, 1, 8, 1, '2024-07-28 12:07:00'),
    ('0xabc131', 9, '0xhash9', '0xfrom9', '0xto9', '0xtoken9', 0.9, 90.0, 0, 9, 0, '2024-07-28 12:08:00'),
    ('0xabc132', 10, '0xhash10', '0xfrom10', '0xto10', '0xtoken10', 1.0, 100.0, 1, 10, 1, '2024-07-28 12:09:00');



-- 插入测试数据
INSERT INTO deposits (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp)
VALUES
    ('0xabc123', 1, '0xhash1', '0xfrom1', '0xto1', '0xtoken1', 0.1, 10.0, 0, 1, '2024-07-28 12:00:00'),
    ('0xabc124', 2, '0xhash2', '0xfrom2', '0xto2', '0xtoken2', 0.2, 20.0, 1, 2, '2024-07-28 12:01:00'),
    ('0xabc125', 3, '0xhash3', '0xfrom3', '0xto3', '0xtoken3', 0.3, 30.0, 0, 3, '2024-07-28 12:02:00'),
    ('0xabc126', 4, '0xhash4', '0xfrom4', '0xto4', '0xtoken4', 0.4, 40.0, 1, 4, '2024-07-28 12:03:00'),
    ('0xabc127', 5, '0xhash5', '0xfrom5', '0xto5', '0xtoken5', 0.5, 50.0, 0, 5, '2024-07-28 12:04:00'),
    ('0xabc128', 6, '0xhash6', '0xfrom6', '0xto6', '0xtoken6', 0.6, 60.0, 1, 6, '2024-07-28 12:05:00'),
    ('0xabc129', 7, '0xhash7', '0xfrom7', '0xto7', '0xtoken7', 0.7, 70.0, 0, 7, '2024-07-28 12:06:00'),
    ('0xabc130', 8, '0xhash8', '0xfrom8', '0xto8', '0xtoken8', 0.8, 80.0, 1, 8, '2024-07-28 12:07:00'),
    ('0xabc131', 9, '0xhash9', '0xfrom9', '0xto9', '0xtoken9', 0.9, 90.0, 0, 9, '2024-07-28 12:08:00'),
    ('0xabc132', 10, '0xhash10', '0xfrom10', '0xto10', '0xtoken10', 1.0, 100.0, 1, 10, '2024-07-28 12:09:00');


-- 插入测试数据
INSERT INTO withdraws (block_hash, block_number, hash, from_address, to_address, token_address, fee, amount, status, transaction_index, timestamp, tx_sign_hex)
VALUES
    ('0xabc123', 1, '0xhash1', '0xfrom1', '0xto1', '0xtoken1', 0.1, 10.0, 0, 1, '2024-07-28 12:00:00', '0xtxsign1'),
    ('0xabc124', 2, '0xhash2', '0xfrom2', '0xto2', '0xtoken2', 0.2, 20.0, 1, 2, '2024-07-28 12:01:00', '0xtxsign2'),
    ('0xabc125', 3, '0xhash3', '0xfrom3', '0xto3', '0xtoken3', 0.3, 30.0, 0, 3, '2024-07-28 12:02:00', '0xtxsign3'),
    ('0xabc126', 4, '0xhash4', '0xfrom4', '0xto4', '0xtoken4', 0.4, 40.0, 1, 4, '2024-07-28 12:03:00', '0xtxsign4'),
    ('0xabc127', 5, '0xhash5', '0xfrom5', '0xto5', '0xtoken5', 0.5, 50.0, 0, 5, '2024-07-28 12:04:00', '0xtxsign5'),
    ('0xabc128', 6, '0xhash6', '0xfrom6', '0xto6', '0xtoken6', 0.6, 60.0, 1, 6, '2024-07-28 12:05:00', '0xtxsign6'),
    ('0xabc129', 7, '0xhash7', '0xfrom7', '0xto7', '0xtoken7', 0.7, 70.0, 0, 7, '2024-07-28 12:06:00', '0xtxsign7'),
    ('0xabc130', 8, '0xhash8', '0xfrom8', '0xto8', '0xtoken8', 0.8, 80.0, 1, 8, '2024-07-28 12:07:00', '0xtxsign8'),
    ('0xabc131', 9, '0xhash9', '0xfrom9', '0xto9', '0xtoken9', 0.9, 90.0, 0, 9, '2024-07-28 12:08:00', '0xtxsign9'),
    ('0xabc132', 10, '0xhash10', '0xfrom10', '0xto10', '0xtoken10', 1.0, 100.0, 1, 10, '2024-07-28 12:09:00', '0xtxsign10');
















