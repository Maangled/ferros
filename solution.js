const http = require('http');

http.get('http://localhost:4317/monitor/state', (res) => {
  let data = '';
  res.on('data', (chunk) => data += chunk);
  res.on('end', () => {
    try {
      const state = JSON.parse(data);
      if (!state.agents || state.agents.length === 0) {
        console.log('No agents found in state.');
        return;
      }

      const categories = {
        software: [],
        hardware: [],
        infrastructure: []
      };

      state.agents.forEach(agent => {
        const cat = (agent.behavior && agent.behavior.category) || 'software';
        if (categories[cat]) {
          categories[cat].push(agent.name);
        } else {
          categories['software'].push(agent.name);
        }
      });

      for (const cat in categories) {
        categories[cat].sort();
        console.log(`${cat}: ${categories[cat].length}`);
        categories[cat].forEach(name => console.log(`- ${name}`));
      }

      const sw = categories.software;
      const sa = sw.indexOf('Software Architect');
      const fc = sw.indexOf('FERROS Core Agent');
      const fsc = sw.indexOf('FERROS SubCore Agent');
      
      if (sa !== -1 && fc !== -1 && fsc !== -1) {
        if (sa < fc && sa < fsc) {
          console.log('Confirm: Software Architect appears before FERROS Core Agent and FERROS SubCore Agent.');
        } else {
          console.log('Confirm: Ordering incorrect.');
        }
      } else {
        console.log('Required agents for confirmation not found in software category.');
      }
    } catch (e) {
      console.error('Error parsing state:', e.message);
    }
  });
}).on('error', (e) => {
  console.error('Fetch error:', e.message);
});
