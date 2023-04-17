CREATE OR REPLACE FUNCTION get_blog_by_options ( 
	p_id INT8 DEFAULT NULL, 
	p_category_id INT8 DEFAULT NULL, 
	p_title TEXT DEFAULT NULL, 
	p_tags TEXT DEFAULT NULL, 
	p_status_sign INT2 DEFAULT NULL ) 
RETURNS TABLE (
	category_id INT8,
	NAME VARCHAR ( 256 ),
	num INT8,
	ID INT8,
	title VARCHAR ( 128 ),
	digest VARCHAR ( 1024 ),
	sketch VARCHAR ( 256 ),
	tags VARCHAR ( 256 ),
	views INT8,
	likes INT8,
	comments INT8,
	create_at TEXT,
	status_sign INT2,
	is_private BOOL ) 
AS $$ 
DECLARE dyn_sql TEXT;
BEGIN
	dyn_sql = '
		select 
			category.id category_id, category.name, category.num, 
			post.id, title, digest, sketch, tags, views, likes, comments, create_at::text, status_sign, is_private
    	from 
			simple_blog_post post
    	left join 
			simple_blog_category category 
    	on 
			category.id = category_id
		where 1 = 1
	';
	IF p_id IS NOT NULL THEN
		dyn_sql := dyn_sql || ' and post.id = $1';
	END IF;
	IF p_category_id IS NOT NULL THEN
		dyn_sql := dyn_sql || ' and post.category_id = $2';
	END IF;
	IF p_title IS NOT NULL THEN
		dyn_sql := dyn_sql || ' and post.title like $3';
	END IF;
	IF p_tags IS NOT NULL THEN
		dyn_sql := dyn_sql || ' and post.tags like $4';
	END IF;
	IF p_status_sign IS NOT NULL THEN
		dyn_sql := dyn_sql || ' and post.status_sign = $5';
	END IF;
	dyn_sql := dyn_sql || ';';
	RAISE DEBUG 'params: % % % % %', p_id, p_category_id, p_title, p_tags, p_status_sign;
	RAISE DEBUG '%', dyn_sql;
	RETURN QUERY EXECUTE dyn_sql USING p_id, p_category_id, '%' || p_title || '%', '%' || p_tags || '%', p_status_sign;
END;
$$ LANGUAGE plpgsql;