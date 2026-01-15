/**
 * ADRScope Interactive Viewer
 * Zero-dependency vanilla JavaScript
 *
 * Security Note: All user-provided content is escaped via escapeHtml().
 * The body_html content is pre-rendered server-side from markdown and
 * is considered trusted content.
 */

(function() {
    'use strict';

    // =========================================================================
    // State Management
    // =========================================================================
    const state = {
        records: ADRSCOPE_DATA.records || [],
        facets: ADRSCOPE_DATA.facets || {},
        graph: ADRSCOPE_DATA.graph || { nodes: [], edges: [] },
        filteredRecords: [],
        selectedId: null,
        selectedIndex: -1,
        currentView: 'list',
        filters: {
            search: '',
            statuses: [],
            category: '',
            author: '',
            project: '',
            tags: [],
            technologies: [],
            dateFrom: '',
            dateTo: ''
        },
        sort: {
            field: 'updated',
            direction: 'desc'
        }
    };

    // =========================================================================
    // DOM Elements
    // =========================================================================
    const elements = {
        search: document.getElementById('search'),
        statusFilters: document.getElementById('status-filters'),
        categoryFilter: document.getElementById('category-filter'),
        authorFilter: document.getElementById('author-filter'),
        projectFilter: document.getElementById('project-filter'),
        tagFilters: document.getElementById('tag-filters'),
        techFilters: document.getElementById('tech-filters'),
        dateFrom: document.getElementById('date-from'),
        dateTo: document.getElementById('date-to'),
        clearFilters: document.getElementById('clear-filters'),
        sortBy: document.getElementById('sort-by'),
        resultCount: document.getElementById('result-count'),
        footerStats: document.getElementById('footer-stats'),
        adrList: document.getElementById('adr-list'),
        cardGrid: document.getElementById('card-grid'),
        timeline: document.getElementById('timeline'),
        graphCanvas: document.getElementById('graph-canvas'),
        detailPanel: document.getElementById('detail-panel'),
        detailContent: document.getElementById('detail-content'),
        closeDetail: document.getElementById('close-detail'),
        prevAdr: document.getElementById('prev-adr'),
        nextAdr: document.getElementById('next-adr'),
        themeToggle: document.getElementById('theme-toggle'),
        shortcutsModal: document.getElementById('shortcuts-modal'),
        closeShortcuts: document.getElementById('close-shortcuts'),
        viewButtons: document.querySelectorAll('.view-btn'),
        viewContainers: {
            list: document.getElementById('list-view'),
            cards: document.getElementById('cards-view'),
            timeline: document.getElementById('timeline-view'),
            graph: document.getElementById('graph-view')
        }
    };

    // =========================================================================
    // Initialization
    // =========================================================================
    function init() {
        initFilters();
        initEventListeners();
        initTheme();
        applyFilters();
        updateFooterStats();
    }

    function initFilters() {
        // Status filters - using DOM methods for safety
        elements.statusFilters.textContent = '';
        state.facets.statuses.forEach(s => {
            const label = document.createElement('label');
            label.className = 'status-chip status-' + s.value;

            const input = document.createElement('input');
            input.type = 'checkbox';
            input.value = s.value;
            input.hidden = true;

            const labelSpan = document.createElement('span');
            labelSpan.className = 'label';
            labelSpan.textContent = s.value;

            const countSpan = document.createElement('span');
            countSpan.className = 'count';
            countSpan.textContent = s.count;

            label.appendChild(input);
            label.appendChild(labelSpan);
            label.appendChild(countSpan);
            elements.statusFilters.appendChild(label);
        });

        // Category filter
        elements.categoryFilter.textContent = '';
        const defaultCat = document.createElement('option');
        defaultCat.value = '';
        defaultCat.textContent = 'All categories';
        elements.categoryFilter.appendChild(defaultCat);

        state.facets.categories.forEach(c => {
            const opt = document.createElement('option');
            opt.value = c.value;
            opt.textContent = c.value + ' (' + c.count + ')';
            elements.categoryFilter.appendChild(opt);
        });

        // Author filter
        elements.authorFilter.textContent = '';
        const defaultAuth = document.createElement('option');
        defaultAuth.value = '';
        defaultAuth.textContent = 'All authors';
        elements.authorFilter.appendChild(defaultAuth);

        state.facets.authors.forEach(a => {
            const opt = document.createElement('option');
            opt.value = a.value;
            opt.textContent = a.value + ' (' + a.count + ')';
            elements.authorFilter.appendChild(opt);
        });

        // Project filter
        elements.projectFilter.textContent = '';
        const defaultProj = document.createElement('option');
        defaultProj.value = '';
        defaultProj.textContent = 'All projects';
        elements.projectFilter.appendChild(defaultProj);

        state.facets.projects.forEach(p => {
            const opt = document.createElement('option');
            opt.value = p.value;
            opt.textContent = p.value + ' (' + p.count + ')';
            elements.projectFilter.appendChild(opt);
        });

        // Tag filters
        elements.tagFilters.textContent = '';
        state.facets.tags.slice(0, 20).forEach(t => {
            const label = document.createElement('label');
            label.className = 'tag-chip';
            label.dataset.value = t.value;
            label.textContent = t.value + ' (' + t.count + ')';
            elements.tagFilters.appendChild(label);
        });

        // Technology filters
        elements.techFilters.textContent = '';
        state.facets.technologies.slice(0, 20).forEach(t => {
            const label = document.createElement('label');
            label.className = 'tag-chip';
            label.dataset.value = t.value;
            label.textContent = t.value + ' (' + t.count + ')';
            elements.techFilters.appendChild(label);
        });
    }

    function initEventListeners() {
        // Search
        let searchTimeout;
        elements.search.addEventListener('input', function() {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(function() {
                state.filters.search = elements.search.value.trim().toLowerCase();
                applyFilters();
            }, 300);
        });

        // Status filters
        elements.statusFilters.addEventListener('click', function(e) {
            const chip = e.target.closest('.status-chip');
            if (chip) {
                chip.classList.toggle('active');
                const value = chip.querySelector('input').value;
                const idx = state.filters.statuses.indexOf(value);
                if (idx > -1) {
                    state.filters.statuses.splice(idx, 1);
                } else {
                    state.filters.statuses.push(value);
                }
                applyFilters();
            }
        });

        // Select filters
        elements.categoryFilter.addEventListener('change', function() {
            state.filters.category = elements.categoryFilter.value;
            applyFilters();
        });

        elements.authorFilter.addEventListener('change', function() {
            state.filters.author = elements.authorFilter.value;
            applyFilters();
        });

        elements.projectFilter.addEventListener('change', function() {
            state.filters.project = elements.projectFilter.value;
            applyFilters();
        });

        // Tag filters
        elements.tagFilters.addEventListener('click', function(e) {
            const chip = e.target.closest('.tag-chip');
            if (chip) {
                chip.classList.toggle('active');
                const value = chip.dataset.value;
                const idx = state.filters.tags.indexOf(value);
                if (idx > -1) {
                    state.filters.tags.splice(idx, 1);
                } else {
                    state.filters.tags.push(value);
                }
                applyFilters();
            }
        });

        // Tech filters
        elements.techFilters.addEventListener('click', function(e) {
            const chip = e.target.closest('.tag-chip');
            if (chip) {
                chip.classList.toggle('active');
                const value = chip.dataset.value;
                const idx = state.filters.technologies.indexOf(value);
                if (idx > -1) {
                    state.filters.technologies.splice(idx, 1);
                } else {
                    state.filters.technologies.push(value);
                }
                applyFilters();
            }
        });

        // Date filters
        elements.dateFrom.addEventListener('change', function() {
            state.filters.dateFrom = elements.dateFrom.value;
            applyFilters();
        });

        elements.dateTo.addEventListener('change', function() {
            state.filters.dateTo = elements.dateTo.value;
            applyFilters();
        });

        // Clear filters
        elements.clearFilters.addEventListener('click', clearAllFilters);

        // Sort
        elements.sortBy.addEventListener('change', function() {
            const parts = elements.sortBy.value.split('-');
            state.sort.field = parts[0];
            state.sort.direction = parts[1];
            applyFilters();
        });

        // View toggle
        elements.viewButtons.forEach(function(btn) {
            btn.addEventListener('click', function() {
                switchView(btn.dataset.view);
            });
        });

        // Detail panel
        elements.closeDetail.addEventListener('click', closeDetail);
        elements.prevAdr.addEventListener('click', navigatePrev);
        elements.nextAdr.addEventListener('click', navigateNext);

        // Theme toggle
        elements.themeToggle.addEventListener('click', toggleTheme);

        // Shortcuts modal
        elements.closeShortcuts.addEventListener('click', function() {
            elements.shortcutsModal.classList.add('hidden');
        });

        // Keyboard navigation
        document.addEventListener('keydown', handleKeydown);

        // Table row clicks
        elements.adrList.addEventListener('click', function(e) {
            const row = e.target.closest('tr');
            if (row && row.dataset.id) {
                selectAdr(row.dataset.id);
            }
        });

        // Card clicks
        elements.cardGrid.addEventListener('click', function(e) {
            const card = e.target.closest('.adr-card');
            if (card && card.dataset.id) {
                selectAdr(card.dataset.id);
            }
        });

        // Timeline clicks
        elements.timeline.addEventListener('click', function(e) {
            const item = e.target.closest('.timeline-item');
            if (item && item.dataset.id) {
                selectAdr(item.dataset.id);
            }
        });
    }

    // =========================================================================
    // Theme Management
    // =========================================================================
    function initTheme() {
        const savedTheme = localStorage.getItem('adrscope-theme');
        if (savedTheme) {
            document.documentElement.dataset.theme = savedTheme;
        }
    }

    function toggleTheme() {
        const html = document.documentElement;
        const current = html.dataset.theme;

        var next;
        if (current === 'light') {
            next = 'dark';
        } else if (current === 'dark') {
            next = 'auto';
        } else {
            next = 'light';
        }

        html.dataset.theme = next;
        localStorage.setItem('adrscope-theme', next);
    }

    // =========================================================================
    // Filtering and Sorting
    // =========================================================================
    function applyFilters() {
        var filtered = state.records.slice();

        // Search filter
        if (state.filters.search) {
            var query = state.filters.search;
            filtered = filtered.filter(function(adr) {
                var searchable = [
                    adr.frontmatter.title,
                    adr.frontmatter.description,
                    (adr.frontmatter.tags || []).join(' '),
                    (adr.frontmatter.technologies || []).join(' '),
                    adr.body_text || ''
                ].join(' ').toLowerCase();
                return searchable.indexOf(query) !== -1;
            });
        }

        // Status filter
        if (state.filters.statuses.length > 0) {
            filtered = filtered.filter(function(adr) {
                return state.filters.statuses.indexOf(adr.frontmatter.status) !== -1;
            });
        }

        // Category filter
        if (state.filters.category) {
            filtered = filtered.filter(function(adr) {
                return adr.frontmatter.category === state.filters.category;
            });
        }

        // Author filter
        if (state.filters.author) {
            filtered = filtered.filter(function(adr) {
                return adr.frontmatter.author === state.filters.author;
            });
        }

        // Project filter
        if (state.filters.project) {
            filtered = filtered.filter(function(adr) {
                return adr.frontmatter.project === state.filters.project;
            });
        }

        // Tag filter
        if (state.filters.tags.length > 0) {
            filtered = filtered.filter(function(adr) {
                return state.filters.tags.every(function(tag) {
                    return (adr.frontmatter.tags || []).indexOf(tag) !== -1;
                });
            });
        }

        // Technology filter
        if (state.filters.technologies.length > 0) {
            filtered = filtered.filter(function(adr) {
                return state.filters.technologies.every(function(tech) {
                    return (adr.frontmatter.technologies || []).indexOf(tech) !== -1;
                });
            });
        }

        // Date range filter
        if (state.filters.dateFrom) {
            filtered = filtered.filter(function(adr) {
                return adr.frontmatter.created >= state.filters.dateFrom;
            });
        }
        if (state.filters.dateTo) {
            filtered = filtered.filter(function(adr) {
                return adr.frontmatter.created <= state.filters.dateTo;
            });
        }

        // Sort
        filtered.sort(function(a, b) {
            var aVal, bVal;
            switch (state.sort.field) {
                case 'title':
                    aVal = a.frontmatter.title.toLowerCase();
                    bVal = b.frontmatter.title.toLowerCase();
                    break;
                case 'status':
                    aVal = a.frontmatter.status;
                    bVal = b.frontmatter.status;
                    break;
                case 'category':
                    aVal = a.frontmatter.category || '';
                    bVal = b.frontmatter.category || '';
                    break;
                case 'author':
                    aVal = a.frontmatter.author || '';
                    bVal = b.frontmatter.author || '';
                    break;
                case 'created':
                    aVal = a.frontmatter.created || '';
                    bVal = b.frontmatter.created || '';
                    break;
                case 'updated':
                default:
                    aVal = a.frontmatter.updated || a.frontmatter.created || '';
                    bVal = b.frontmatter.updated || b.frontmatter.created || '';
                    break;
            }

            var cmp = 0;
            if (aVal < bVal) cmp = -1;
            else if (aVal > bVal) cmp = 1;

            return state.sort.direction === 'asc' ? cmp : -cmp;
        });

        state.filteredRecords = filtered;

        // Update selected index
        if (state.selectedId) {
            state.selectedIndex = filtered.findIndex(function(r) {
                return r.id === state.selectedId;
            });
        }

        render();
    }

    function clearAllFilters() {
        state.filters = {
            search: '',
            statuses: [],
            category: '',
            author: '',
            project: '',
            tags: [],
            technologies: [],
            dateFrom: '',
            dateTo: ''
        };

        // Reset UI
        elements.search.value = '';
        elements.categoryFilter.value = '';
        elements.authorFilter.value = '';
        elements.projectFilter.value = '';
        elements.dateFrom.value = '';
        elements.dateTo.value = '';

        document.querySelectorAll('.status-chip.active, .tag-chip.active')
            .forEach(function(el) { el.classList.remove('active'); });

        applyFilters();
    }

    // =========================================================================
    // Rendering
    // =========================================================================
    function render() {
        updateResultCount();
        renderListView();
        renderCardView();
        renderTimelineView();
        if (state.currentView === 'graph') {
            renderGraphView();
        }
    }

    function updateResultCount() {
        var total = state.records.length;
        var filtered = state.filteredRecords.length;

        if (filtered === total) {
            elements.resultCount.textContent = total + ' records';
        } else {
            elements.resultCount.textContent = filtered + ' of ' + total + ' records';
        }
    }

    function updateFooterStats() {
        var meta = ADRSCOPE_DATA.meta;
        var generated = new Date(meta.generated).toLocaleString();
        elements.footerStats.textContent = state.records.length + ' ADRs | Generated: ' + generated;
    }

    function renderListView() {
        elements.adrList.textContent = '';

        if (state.filteredRecords.length === 0) {
            var tr = document.createElement('tr');
            var td = document.createElement('td');
            td.setAttribute('colspan', '6');
            td.style.textAlign = 'center';
            td.style.color = 'var(--color-text-muted)';
            td.textContent = 'No ADRs match the current filters';
            tr.appendChild(td);
            elements.adrList.appendChild(tr);
            return;
        }

        state.filteredRecords.forEach(function(adr) {
            var tr = document.createElement('tr');
            tr.dataset.id = adr.id;
            if (adr.id === state.selectedId) {
                tr.className = 'selected';
            }

            // Status cell
            var tdStatus = document.createElement('td');
            var statusBadge = document.createElement('span');
            statusBadge.className = 'status-badge status-' + adr.frontmatter.status;
            statusBadge.textContent = adr.frontmatter.status;
            tdStatus.appendChild(statusBadge);
            tr.appendChild(tdStatus);

            // Title cell
            var tdTitle = document.createElement('td');
            tdTitle.className = 'title-cell';
            tdTitle.textContent = adr.frontmatter.title;
            tr.appendChild(tdTitle);

            // Category cell
            var tdCategory = document.createElement('td');
            var catBadge = document.createElement('span');
            catBadge.className = 'category-badge';
            catBadge.textContent = adr.frontmatter.category || '-';
            tdCategory.appendChild(catBadge);
            tr.appendChild(tdCategory);

            // Author cell
            var tdAuthor = document.createElement('td');
            tdAuthor.textContent = adr.frontmatter.author || '-';
            tr.appendChild(tdAuthor);

            // Created cell
            var tdCreated = document.createElement('td');
            tdCreated.className = 'date-cell';
            tdCreated.textContent = formatDate(adr.frontmatter.created);
            tr.appendChild(tdCreated);

            // Updated cell
            var tdUpdated = document.createElement('td');
            tdUpdated.className = 'date-cell';
            if (isRecent(adr.frontmatter.updated)) {
                tdUpdated.classList.add('recent');
            }
            tdUpdated.textContent = formatDate(adr.frontmatter.updated);
            tr.appendChild(tdUpdated);

            elements.adrList.appendChild(tr);
        });
    }

    function renderCardView() {
        elements.cardGrid.textContent = '';

        if (state.filteredRecords.length === 0) {
            var p = document.createElement('p');
            p.style.textAlign = 'center';
            p.style.color = 'var(--color-text-muted)';
            p.textContent = 'No ADRs match the current filters';
            elements.cardGrid.appendChild(p);
            return;
        }

        state.filteredRecords.forEach(function(adr) {
            var card = document.createElement('div');
            card.className = 'adr-card';
            card.dataset.id = adr.id;
            if (adr.id === state.selectedId) {
                card.classList.add('selected');
            }

            // Status
            var statusDiv = document.createElement('div');
            statusDiv.className = 'card-status';
            var statusBadge = document.createElement('span');
            statusBadge.className = 'status-badge status-' + adr.frontmatter.status;
            statusBadge.textContent = adr.frontmatter.status;
            statusDiv.appendChild(statusBadge);
            card.appendChild(statusDiv);

            // Title
            var title = document.createElement('h3');
            title.className = 'card-title';
            title.textContent = adr.frontmatter.title;
            card.appendChild(title);

            // Description
            var desc = document.createElement('p');
            desc.className = 'card-description';
            desc.textContent = adr.frontmatter.description || '';
            card.appendChild(desc);

            // Meta
            var meta = document.createElement('div');
            meta.className = 'card-meta';
            var catBadge = document.createElement('span');
            catBadge.className = 'category-badge';
            catBadge.textContent = adr.frontmatter.category || 'uncategorized';
            meta.appendChild(catBadge);
            card.appendChild(meta);

            // Tags
            var tagsDiv = document.createElement('div');
            tagsDiv.className = 'card-tags';
            (adr.frontmatter.tags || []).slice(0, 3).forEach(function(tag) {
                var tagSpan = document.createElement('span');
                tagSpan.className = 'card-tag';
                tagSpan.textContent = tag;
                tagsDiv.appendChild(tagSpan);
            });
            card.appendChild(tagsDiv);

            // Footer
            var footer = document.createElement('div');
            footer.className = 'card-footer';
            var authorSpan = document.createElement('span');
            authorSpan.textContent = adr.frontmatter.author || 'Unknown';
            var dateSpan = document.createElement('span');
            dateSpan.textContent = formatDate(adr.frontmatter.created);
            footer.appendChild(authorSpan);
            footer.appendChild(dateSpan);
            card.appendChild(footer);

            elements.cardGrid.appendChild(card);
        });
    }

    function renderTimelineView() {
        elements.timeline.textContent = '';

        if (state.filteredRecords.length === 0) {
            var p = document.createElement('p');
            p.style.textAlign = 'center';
            p.style.color = 'var(--color-text-muted)';
            p.textContent = 'No ADRs match the current filters';
            elements.timeline.appendChild(p);
            return;
        }

        // Group by month
        var groups = {};
        state.filteredRecords.forEach(function(adr) {
            var date = adr.frontmatter.created;
            var key = date ? date.substring(0, 7) : 'undated';
            if (!groups[key]) groups[key] = [];
            groups[key].push(adr);
        });

        // Sort keys (newest first)
        var sortedKeys = Object.keys(groups).sort().reverse();

        sortedKeys.forEach(function(key) {
            var groupDiv = document.createElement('div');
            groupDiv.className = 'timeline-group';

            var monthDiv = document.createElement('div');
            monthDiv.className = 'timeline-month';
            monthDiv.textContent = key === 'undated' ? 'Undated' : formatMonth(key);
            groupDiv.appendChild(monthDiv);

            groups[key].forEach(function(adr) {
                var item = document.createElement('div');
                item.className = 'timeline-item status-' + adr.frontmatter.status;
                item.dataset.id = adr.id;
                if (adr.id === state.selectedId) {
                    item.classList.add('selected');
                }

                var titleDiv = document.createElement('div');
                titleDiv.className = 'timeline-title';
                titleDiv.textContent = adr.frontmatter.title;
                item.appendChild(titleDiv);

                var dateDiv = document.createElement('div');
                dateDiv.className = 'timeline-date';
                dateDiv.textContent = formatDate(adr.frontmatter.created);
                item.appendChild(dateDiv);

                groupDiv.appendChild(item);
            });

            elements.timeline.appendChild(groupDiv);
        });
    }

    function renderGraphView() {
        var canvas = elements.graphCanvas;
        var ctx = canvas.getContext('2d');
        var rect = canvas.parentElement.getBoundingClientRect();

        canvas.width = rect.width;
        canvas.height = rect.height;

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        if (state.graph.nodes.length === 0) {
            ctx.fillStyle = getComputedStyle(document.documentElement).getPropertyValue('--color-text-muted');
            ctx.font = '14px sans-serif';
            ctx.textAlign = 'center';
            ctx.fillText('No relationships to display', canvas.width / 2, canvas.height / 2);
            return;
        }

        // Simple force-directed layout simulation
        var nodes = state.graph.nodes.map(function(n, i) {
            return {
                id: n.id,
                status: n.status,
                x: canvas.width / 2 + Math.cos(i * 2 * Math.PI / state.graph.nodes.length) * 200,
                y: canvas.height / 2 + Math.sin(i * 2 * Math.PI / state.graph.nodes.length) * 200,
                radius: 20
            };
        });

        var nodeMap = {};
        nodes.forEach(function(n) { nodeMap[n.id] = n; });

        // Draw edges
        ctx.strokeStyle = getComputedStyle(document.documentElement).getPropertyValue('--color-border');
        ctx.lineWidth = 1;

        state.graph.edges.forEach(function(edge) {
            var source = nodeMap[edge.source];
            var target = nodeMap[edge.target];
            if (source && target) {
                ctx.beginPath();
                ctx.moveTo(source.x, source.y);
                ctx.lineTo(target.x, target.y);
                ctx.stroke();
            }
        });

        // Draw nodes
        var statusColors = {
            proposed: '#f59e0b',
            accepted: '#10b981',
            deprecated: '#ef4444',
            superseded: '#6b7280'
        };

        nodes.forEach(function(node) {
            ctx.beginPath();
            ctx.arc(node.x, node.y, node.radius, 0, 2 * Math.PI);
            ctx.fillStyle = statusColors[node.status] || '#6b7280';
            ctx.fill();

            ctx.fillStyle = '#ffffff';
            ctx.font = 'bold 10px sans-serif';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            ctx.fillText(node.id.replace('adr_', ''), node.x, node.y);
        });
    }

    // =========================================================================
    // Detail Panel
    // =========================================================================
    function selectAdr(id) {
        var adr = state.filteredRecords.find(function(r) { return r.id === id; });
        if (!adr) return;

        state.selectedId = id;
        state.selectedIndex = state.filteredRecords.findIndex(function(r) { return r.id === id; });

        // Update selection in views
        document.querySelectorAll('[data-id].selected').forEach(function(el) { el.classList.remove('selected'); });
        document.querySelectorAll('[data-id="' + id + '"]').forEach(function(el) { el.classList.add('selected'); });

        // Render detail content
        renderDetail(adr);

        // Show panel
        elements.detailPanel.classList.remove('hidden');

        // Update navigation
        elements.prevAdr.disabled = state.selectedIndex <= 0;
        elements.nextAdr.disabled = state.selectedIndex >= state.filteredRecords.length - 1;
    }

    function renderDetail(adr) {
        var fm = adr.frontmatter;
        elements.detailContent.textContent = '';

        // Title row with status
        var titleDiv = document.createElement('div');
        titleDiv.className = 'detail-title';
        var statusBadge = document.createElement('span');
        statusBadge.className = 'status-badge status-' + fm.status;
        statusBadge.textContent = fm.status;
        titleDiv.appendChild(statusBadge);
        titleDiv.appendChild(document.createTextNode(' ' + fm.title));
        elements.detailContent.appendChild(titleDiv);

        // Meta grid
        var metaDiv = document.createElement('div');
        metaDiv.className = 'detail-meta';

        var metaItems = [
            { label: 'Category', value: fm.category || '-' },
            { label: 'Author', value: fm.author || '-' },
            { label: 'Project', value: fm.project || '-' },
            { label: 'Created', value: formatDate(fm.created) },
            { label: 'Updated', value: formatDate(fm.updated) },
            { label: 'Tags', value: (fm.tags || []).join(', ') || '-' }
        ];

        metaItems.forEach(function(item) {
            var itemDiv = document.createElement('div');
            itemDiv.className = 'meta-item';
            var labelSpan = document.createElement('span');
            labelSpan.className = 'meta-label';
            labelSpan.textContent = item.label;
            var valueSpan = document.createElement('span');
            valueSpan.className = 'meta-value';
            valueSpan.textContent = item.value;
            itemDiv.appendChild(labelSpan);
            itemDiv.appendChild(valueSpan);
            metaDiv.appendChild(itemDiv);
        });
        elements.detailContent.appendChild(metaDiv);

        // Body HTML (trusted content from server-side markdown rendering)
        var bodyDiv = document.createElement('div');
        bodyDiv.className = 'detail-body';
        // body_html is pre-rendered markdown from the server - trusted content
        bodyDiv.innerHTML = adr.body_html;
        elements.detailContent.appendChild(bodyDiv);

        // Related ADRs
        if (fm.related && fm.related.length > 0) {
            var relatedDiv = document.createElement('div');
            relatedDiv.className = 'detail-related';
            var relatedH3 = document.createElement('h3');
            relatedH3.textContent = 'Related ADRs';
            relatedDiv.appendChild(relatedH3);

            var relatedList = document.createElement('div');
            relatedList.className = 'related-list';

            fm.related.forEach(function(r) {
                var link = document.createElement('a');
                link.href = '#';
                link.className = 'related-link';
                var relId = r.replace('.md', '');
                link.dataset.id = relId;
                link.textContent = r;
                link.addEventListener('click', function(e) {
                    e.preventDefault();
                    var relAdr = state.filteredRecords.find(function(rec) { return rec.id === relId; });
                    if (relAdr) {
                        selectAdr(relId);
                    }
                });
                relatedList.appendChild(link);
            });

            relatedDiv.appendChild(relatedList);
            elements.detailContent.appendChild(relatedDiv);
        }
    }

    function closeDetail() {
        elements.detailPanel.classList.add('hidden');
        state.selectedId = null;
        state.selectedIndex = -1;
        document.querySelectorAll('[data-id].selected').forEach(function(el) { el.classList.remove('selected'); });
    }

    function navigatePrev() {
        if (state.selectedIndex > 0) {
            selectAdr(state.filteredRecords[state.selectedIndex - 1].id);
        }
    }

    function navigateNext() {
        if (state.selectedIndex < state.filteredRecords.length - 1) {
            selectAdr(state.filteredRecords[state.selectedIndex + 1].id);
        }
    }

    // =========================================================================
    // View Switching
    // =========================================================================
    function switchView(view) {
        state.currentView = view;

        // Update buttons
        elements.viewButtons.forEach(function(btn) {
            var isActive = btn.dataset.view === view;
            btn.classList.toggle('active', isActive);
            btn.setAttribute('aria-selected', isActive);
        });

        // Update containers
        Object.keys(elements.viewContainers).forEach(function(key) {
            elements.viewContainers[key].classList.toggle('hidden', key !== view);
        });

        // Render graph on switch
        if (view === 'graph') {
            requestAnimationFrame(renderGraphView);
        }
    }

    // =========================================================================
    // Keyboard Navigation
    // =========================================================================
    function handleKeydown(e) {
        // Ignore when typing in input
        if (e.target.matches('input, select, textarea')) {
            if (e.key === 'Escape') {
                e.target.blur();
            }
            return;
        }

        switch (e.key) {
            case '/':
                e.preventDefault();
                elements.search.focus();
                break;

            case 'Escape':
                if (!elements.detailPanel.classList.contains('hidden')) {
                    closeDetail();
                } else if (!elements.shortcutsModal.classList.contains('hidden')) {
                    elements.shortcutsModal.classList.add('hidden');
                } else {
                    clearAllFilters();
                }
                break;

            case 'j':
            case 'ArrowDown':
                e.preventDefault();
                if (state.selectedIndex < state.filteredRecords.length - 1) {
                    selectAdr(state.filteredRecords[state.selectedIndex + 1].id);
                } else if (state.filteredRecords.length > 0 && state.selectedIndex === -1) {
                    selectAdr(state.filteredRecords[0].id);
                }
                break;

            case 'k':
            case 'ArrowUp':
                e.preventDefault();
                if (state.selectedIndex > 0) {
                    selectAdr(state.filteredRecords[state.selectedIndex - 1].id);
                }
                break;

            case 'Enter':
                if (state.selectedId && elements.detailPanel.classList.contains('hidden')) {
                    selectAdr(state.selectedId);
                }
                break;

            case '1':
                switchView('list');
                break;
            case '2':
                switchView('cards');
                break;
            case '3':
                switchView('timeline');
                break;
            case '4':
                switchView('graph');
                break;

            case '?':
                e.preventDefault();
                elements.shortcutsModal.classList.toggle('hidden');
                break;
        }
    }

    // =========================================================================
    // Utility Functions
    // =========================================================================
    function formatDate(dateStr) {
        if (!dateStr) return '-';
        try {
            var date = new Date(dateStr);
            return date.toLocaleDateString('en-US', {
                year: 'numeric',
                month: 'short',
                day: 'numeric'
            });
        } catch (e) {
            return dateStr;
        }
    }

    function formatMonth(key) {
        var parts = key.split('-');
        var date = new Date(parts[0], parseInt(parts[1], 10) - 1);
        return date.toLocaleDateString('en-US', { year: 'numeric', month: 'long' });
    }

    function isRecent(dateStr) {
        if (!dateStr) return false;
        var date = new Date(dateStr);
        var now = new Date();
        var diffDays = (now - date) / (1000 * 60 * 60 * 24);
        return diffDays <= 30;
    }

    // =========================================================================
    // Start Application
    // =========================================================================
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
