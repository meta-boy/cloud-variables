-- Seed default tiers
INSERT INTO tiers (id, name, description, max_variables, max_variable_size_mb, max_requests_per_day, max_api_keys, price_monthly, is_active)
VALUES
    (
        '00000000-0000-0000-0000-000000000001',
        'free',
        'Free tier with basic features',
        10,
        1,  -- 1 MB
        1000,
        2,
        0,
        true
    ),
    (
        '00000000-0000-0000-0000-000000000002',
        'basic',
        'Basic tier for individual developers',
        50,
        10,  -- 10 MB
        10000,
        5,
        999,  -- $9.99/month
        true
    ),
    (
        '00000000-0000-0000-0000-000000000003',
        'pro',
        'Professional tier for teams and production apps',
        200,
        100,  -- 100 MB
        100000,
        20,
        2999,  -- $29.99/month
        true
    ),
    (
        '00000000-0000-0000-0000-000000000004',
        'enterprise',
        'Enterprise tier with unlimited features',
        -1,  -- Unlimited
        -1,  -- Unlimited
        -1,  -- Unlimited
        -1,  -- Unlimited
        9999,  -- $99.99/month
        true
    )
ON CONFLICT (name) DO NOTHING;
