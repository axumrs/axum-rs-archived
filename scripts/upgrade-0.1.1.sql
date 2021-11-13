DROP VIEW v_subject_topics;
CREATE VIEW v_subject_topics AS
SELECT v.id,v.title,v.slug,subject_slug,tag_names,subject_name,  t.summary
 FROM v_topic_subject_list_with_tags AS v
INNER JOIN topic AS t on t.id=v.id
INNER JOIN topic_content AS c ON c.topic_id=v.id
WHERE v.is_del=false AND v.subject_is_del=false;
