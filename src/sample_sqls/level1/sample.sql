select 
u.*
, r.* 
from {{ ref("db", "users") }} as u
left join {{ ref("role") }} as r on
u.id = r.user_id