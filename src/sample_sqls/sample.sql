select 
u.*
, r.* 
from {{ ref("db", "users") }} as u
left join {{ ref("db", "role") }} as r on
u.id = r.user_id